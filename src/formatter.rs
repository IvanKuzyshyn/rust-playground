use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
enum Method {
    DELETE,
    GET,
    OPTIONS,
    PATCH,
    POST,
    PUT,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Method::DELETE => write!(f, "DELETE"),
            Method::GET => write!(f, "GET"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::PATCH => write!(f, "PATCH"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
        }
    }
}

#[derive(Debug)]
struct Request {
    method: Method,
    host: &'static str,
    url: &'static str,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f, "{} request to {}{}", self.method, self.url, self.host)
    }
}

pub fn run() {
    let request = Request {
        method: Method::GET,
        host: "https://example.com",
        url: "/api/example",
    };

    println!("Test request is: {:#?}", request);
}
