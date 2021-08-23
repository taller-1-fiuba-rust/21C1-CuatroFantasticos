use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::fs;

/// PageNotFoundHandler
///
/// Shows a page that displays a 404 error, due to a page not found
///
pub struct PageNotFoundHandler {}

impl PageNotFoundHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        let file = fs::read_to_string("http_server/src/server/handler/page_not_found/404.html");
        HttpResponse::new("404", None, file.ok())
    }
}
