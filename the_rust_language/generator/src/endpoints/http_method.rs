#[derive(Debug)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
}

impl HttpMethod {
    pub fn new(method: &str) -> HttpMethod {
        match method {
            "get" | "GET" => Self::GET,
            "put" | "PUT" => Self::PUT,
            "post" | "POST" => Self::POST,
            "delete" | "DELETE" => Self::DELETE,
            "options" | "OPTIONS" => Self::OPTIONS,
            unknown_method => {
                println!("Warning : Unknown HTTP method {unknown_method} ");
                Self::GET
            }
        }
    }
}
