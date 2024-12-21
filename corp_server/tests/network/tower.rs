use async_io::Timer;
use pin_project::pin_project;
use rmp_serde::encode;
use std::{
    error::Error,
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
struct Timeout<T> {
    inner: T,
    timeout: Duration,
}

impl<T> Timeout<T> {
    pub fn new(inner: T, timeout: Duration) -> Self {
        Self { inner, timeout }
    }
}

impl<S, Request> Service<Request> for Timeout<S>
where
    S: Service<Request>,
    S::Error: Into<BoxError>,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = TimeoutFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let response_future = self.inner.call(request);
        let sleep = Timer::after(self.timeout);

        TimeoutFuture {
            response_future,
            sleep,
        }
    }
}

#[pin_project]
pub struct TimeoutFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Timer,
}

impl<F, Response, Error> Future for TimeoutFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.response_future.poll(cx) {
            Poll::Ready(result) => {
                // The inner service has a response ready for us, or it has failed.
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {
                // Not quite ready yet...
            }
        }

        match this.sleep.poll(cx) {
            Poll::Ready(_instant) => {
                let error = Box::new(TimeoutError(()));
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {
                // Still some time remaining...
            }
        }

        Poll::Pending
    }
}

#[derive(Debug, Default)]
pub struct TimeoutError(());

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("future timed out")
    }
}

impl Error for TimeoutError {}

pub type BoxError = Box<dyn Error + Send + Sync>;

impl<S> Layer<S> for Timeout<S> {
    type Service = Timeout<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Timeout {
            inner,
            timeout: self.timeout,
        }
    }
}

struct MessagePack;

impl MessagePack {
    pub fn new() -> Self {
        MessagePack
    }
}

impl<T> Service<T> for MessagePack
where
    T: Into<String> + Send + 'static,
{
    type Response = Vec<u8>;
    type Error = BoxError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: T) -> Self::Future {
        let input = req.into();
        Box::pin(async move {
            // Add some lag
            // Timer::after(Duration::from_millis(150)).await;
            let bytes =
                encode::to_vec(&input).map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
            Ok(bytes)
        })
    }
}

#[cfg(test)]
mod tests {
    use bevy::{log::LogPlugin, prelude::*, tasks::IoTaskPool};
    use std::time::Duration;

    #[test]
    fn bevy_tasks_block_on() {
        use async_io::Timer;
        use std::time::Duration;

        bevy::tasks::block_on(async {
            // This timer will likely be processed by the current
            // thread rather than the fallback "async-io" thread.
            Timer::after(Duration::from_millis(1)).await;
            println!("Hello World!");
        });
    }

    #[test]
    fn bevy_async_io() {
        enum AppChannel {
            Startup,
            Task(String),
        }

        let (tx, rx) = async_channel::unbounded::<AppChannel>();

        let mut app = App::new();
        app.add_plugins((LogPlugin::default(), MinimalPlugins))
            .add_systems(Startup, |mut commands: Commands| {
                let io_pool = IoTaskPool::get();
                let task = io_pool
                    .spawn(async {
                        println!("Hello World!");
                    })
                    .detach();
            });

        app.update();
    }
    #[test]
    fn do_test() {
        // async_io::block_on(async {
        //     let mp_service = MessagePack::new();
        //     let timeout_service = Timeout::new(mp_service, Duration::from_millis(100));
        //     let mut service = ServiceBuilder::new().service(timeout_service);
        //
        //     match service.call("Hello World!").await {
        //         Ok(response) => {
        //             println!("Service response ok: {:?}", response);
        //         }
        //         Err(err) => {
        //             println!("Service response err: {:?}", err);
        //         }
        //     }
        //
        //     assert_eq!(1 + 1, 2);
        // });
    }
}
