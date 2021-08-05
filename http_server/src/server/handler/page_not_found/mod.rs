use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use std::fs;

/// Devuelve un aviso de pagina no encontrada
pub struct PageNotFoundHandler {}

impl PageNotFoundHandler {
    pub fn handle(_request: &HttpRequest) -> HttpResponse {
        println!("Entro en 404");
        let file = fs::read_to_string("404.html");
        println!("{:?}", file);
        HttpResponse::new("404", None, file.ok())
    }
}
