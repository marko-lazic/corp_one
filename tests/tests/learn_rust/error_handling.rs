use std::{
    fs::{self, File},
    io::Error,
    path::Path,
};
use std::io::ErrorKind;

const FAILED_TO_OPEN_NOFILE_TXT_MESSAGE: &str = "Failed to open nofile.txt";

#[test]
#[should_panic]
fn file_not_found() {
    let f = File::open("nofile.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file: {:?}", error),
    };
}

#[test]
fn failing_more_gracefuly() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem openig the file {:?}", other_error)
            }
        },
    };
    assert!(Path::new("hello.txt").exists());
    let _ = fs::remove_file("hello.txt");
}

#[test]
fn failing_more_gracefuly_closures() {
    let _ = fs::remove_file("hello.txt");
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file {:?}", error))
        } else {
            panic!("Problem openig the file {:?}", error)
        }
    });
    assert!(Path::new("hello.txt").exists());
    let _ = fs::remove_file("hello.txt");
}

#[test]
#[should_panic]
fn unwrap_file_from_result() {
    let _f = File::open("nofile.txt").unwrap();
}

#[test]
#[should_panic(expected = "Failed to open nofile.txt")]
fn specify_fail_message() {
    let _f = File::open("nofile.txt").expect(FAILED_TO_OPEN_NOFILE_TXT_MESSAGE);
}

#[test]
#[should_panic]
fn error_propagation() {
    match open_file() {
        Ok(_) => println!("File is opened succssessfuly"),
        Err(e) => panic!(
            "Not able to open file Here is the reason {:?}",
            e.to_string()
        ),
    }
}

fn open_file() -> Result<(), Error> {
    let file: Result<File, Error> = File::open("hello.txt");
    match file {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
