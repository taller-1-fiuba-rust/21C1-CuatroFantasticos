use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::fs;

/// DefaultHandler
///
/// Shows the default menu screen (whenever a get method is sent)
pub struct DefaultHandler {}

impl DefaultHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        let file = fs::read_to_string("http_server/src/server/handler/default/default.html");
        HttpResponse::new("200", None, file.ok())
    }
}
