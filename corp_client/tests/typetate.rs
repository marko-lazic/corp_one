#[test]
fn a_simple_response() {
    HttpResponse::new()
        .status_line(200, "OK")
        .header("X-Unexpected", "Spanish-Inquisition")
        .header("Content-Length", "6")
        .body("Hello!");
}

struct HttpResponse<S: ResponseState> {
    state: Box<ActualResponseState>,
    extra: S,
}

struct ActualResponseState {
    // cool: i32
}

struct Start {}

struct Headers {
    response_code: u8,
}

trait ResponseState {}

impl ResponseState for Start {}

impl ResponseState for Headers {}

impl HttpResponse<Start> {
    fn new() -> Self {
        HttpResponse::<Start> {
            state: Box::new(ActualResponseState {}),
            extra: Start {},
        }
    }

    fn status_line(self, response_code: u8, message: &str) -> HttpResponse<Headers> {
        // Capture the response code in the new state.
        // In an actual HTTP implementation you'd
        // probably also want to send some data. ;-)
        println!("status line message {}", message);
        HttpResponse {
            state: self.state,
            extra: Headers { response_code },
        }
    }
}

impl HttpResponse<Headers> {
    fn response_code(&self) -> u8 {
        self.extra.response_code
    }

    fn header(self, key: &str, value: &str) -> Self {
        println!("header key {} value {}", key, value);
        self
    }

    fn body(self, contents: &str) {
        println!("Http response body called!");
        println!(
            "response code from HttpResponse<Start> state {}",
            self.response_code()
        );
        println!("Contents {} ", contents);
    }
}
