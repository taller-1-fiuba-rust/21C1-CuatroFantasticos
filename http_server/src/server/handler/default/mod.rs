use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::fs;

/// Devuelve la pagina principal
pub struct DefaultHandler {}

impl DefaultHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        let file = fs::read_to_string("http_server/src/server/handler/default/default.html");
        HttpResponse::new("200", None, file.ok())
    }
}
