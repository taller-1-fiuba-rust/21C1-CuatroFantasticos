use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

/// Devuelve la pagina principal
pub struct DefaultHandler {}

impl DefaultHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        todo!()
    }
}
