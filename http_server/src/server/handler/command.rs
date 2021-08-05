use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;

/// Ejecuta un comando
pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        todo!()
    }
}
