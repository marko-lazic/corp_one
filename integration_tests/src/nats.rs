#![allow(dead_code)]
use std::{
    env, fs,
    io::{BufRead, BufReader},
    net::TcpStream,
    path::PathBuf,
    process::{Child, Command},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use regex::Regex;
use serde_json::{self, Value};

pub struct Server {
    inner: Inner,
}

struct Inner {
    cfg: String,
    id: String,
    port: Option<String>,
    child: Child,
    logfile: PathBuf,
    pidfile: PathBuf,
}

lazy_static! {
    static ref SD_RE: Regex = Regex::new(r#".+\sStore Directory:\s+"([^"]+)""#).unwrap();
    static ref CLIENT_RE: Regex = Regex::new(r".+\sclient connections on\s+(\S+)").unwrap();
}

impl Drop for Server {
    fn drop(&mut self) {
        self.inner.child.kill().unwrap();
        self.inner.child.wait().unwrap();
        if let Ok(log) = fs::read_to_string(self.inner.logfile.as_os_str()) {
            // Check if we had JetStream running and if so cleanup the storage directory.
            if let Some(caps) = SD_RE.captures(&log) {
                let sd = caps.get(1).map_or("", |m| m.as_str());
                fs::remove_dir_all(sd).ok();
            }
            // Remove Logfile.
            fs::remove_file(self.inner.logfile.as_os_str()).ok();
        }
    }
}

impl Server {
    pub fn restart(&mut self) {
        let port = self
            .inner
            .port
            .clone()
            .expect("can't restart server with dynamic port");
        self.inner.child.kill().unwrap();
        self.inner.child.wait().unwrap();
        let inner = do_run(&self.inner.cfg, Some(&port), Some(self.inner.id.clone()));
        self.inner = inner;
    }

    // Grab client url.
    // Helpful when dynamically allocating ports with -1.
    pub fn client_url(&self) -> String {
        let addr = self.client_addr();
        let mut r = BufReader::with_capacity(1024, TcpStream::connect(addr).unwrap());
        let mut line = String::new();
        r.read_line(&mut line).expect("did not receive INFO");
        let si: Value = serde_json::from_str(&line["INFO".len()..]).expect("could not parse INFO");
        let port = si["port"].as_u64().expect("could not parse port") as u16;
        let mut scheme = "nats://";
        if si["tls_required"].as_bool().unwrap_or(false) {
            scheme = "tls://";
        }
        format!("{scheme}localhost:{port}")
    }

    pub fn client_port(&self) -> u16 {
        let addr = self.client_addr();
        let mut r = BufReader::with_capacity(1024, TcpStream::connect(addr).unwrap());
        let mut line = String::new();
        r.read_line(&mut line).expect("did not receive INFO");
        let si: Value = serde_json::from_str(&line["INFO".len()..]).expect("could not parse INFO");
        si["port"].as_u64().expect("could not parse port") as u16
    }

    // Grab client addr from logs.
    fn client_addr(&self) -> String {
        // We may need to wait for log to be present.
        // Wait up to 10s. (100 * 100ms)
        for _ in 0..100 {
            match fs::read_to_string(self.inner.logfile.as_os_str()) {
                Ok(l) => {
                    if let Some(cre) = CLIENT_RE.captures(&l) {
                        return cre.get(1).unwrap().as_str().replace("0.0.0.0", "localhost");
                    } else {
                        thread::sleep(Duration::from_millis(500));
                    }
                }
                _ => thread::sleep(Duration::from_millis(500)),
            }
        }
        panic!("no client addr info");
    }

    pub fn client_pid(&self) -> usize {
        String::from_utf8(fs::read(self.inner.pidfile.clone()).unwrap())
            .unwrap()
            .parse()
            .unwrap()
    }
}

pub struct Config<'a>([&'a str; 3]);

pub trait IntoConfig<'a> {
    fn into_config(self) -> Config<'a>;
}

impl<'a> IntoConfig<'a> for &'a str {
    fn into_config(self) -> Config<'a> {
        Config([self, self, self])
    }
}

impl<'a> IntoConfig<'a> for [&'a str; 3] {
    fn into_config(self) -> Config<'a> {
        Config(self)
    }
}

/// Starts a local basic NATS server that gets stopped and cleaned up on drop.
pub fn run_basic_server() -> Server {
    run_server("")
}
pub fn run_server(cfg: &str) -> Server {
    run_server_with_port(cfg, Some("50961"))
}

/// Starts a local NATS server with the given config that gets stopped and cleaned up on drop.
pub fn run_server_with_port(cfg: &str, port: Option<&str>) -> Server {
    Server {
        inner: do_run(cfg, port, None),
    }
}

fn do_run(cfg: &str, port: Option<&str>, id: Option<String>) -> Inner {
    let id = id.unwrap_or_else(|| nuid::next().to_string());
    let logfile = env::temp_dir().join(format!("nats-server-{id}.log"));
    let pidfile = env::temp_dir().join(format!("nats-server-{id}.pid"));
    let store_dir = env::temp_dir().join(format!("store-dir-{id}"));

    // Always use dynamic ports so tests can run in parallel.
    // Create env for a storage directory for jetstream.
    let mut cmd = Command::new("nats-server");
    cmd.arg("--jetstream")
        .arg("--store_dir")
        .arg(store_dir.as_path().to_str().unwrap())
        .arg("-p");
    match port {
        Some(port) => cmd.arg(port),
        None => cmd.arg("-1"),
    };
    cmd.arg("-l")
        .arg(logfile.as_os_str())
        .arg("-P")
        .arg(pidfile.as_os_str());

    if !cfg.is_empty() {
        cmd.arg("-c").arg(cfg);
    }

    let child = cmd.spawn().unwrap();
    Inner {
        port: port.map(ToString::to_string),
        cfg: cfg.to_string(),
        id,
        child,
        logfile,
        pidfile,
    }
}
