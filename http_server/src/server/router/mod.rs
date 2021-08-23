use crate::http;
use crate::http::request::method::Method;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::server::handler::command::CommandHandler;
use crate::server::handler::default::DefaultHandler;
use crate::server::handler::page_not_found::PageNotFoundHandler;
use std::io::Write;

pub struct Router;
impl Router {
    pub fn route<T: Write>(req: HttpRequest, stream: &mut T) {
        match req.method {
            Method::Get => match &req.resource {
                http::request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        "" => {
                            let resp: HttpResponse = DefaultHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            Method::Post => match &req.resource {
                http::request::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route[1] {
                        "execute_command" => {
                            let resp: HttpResponse = CommandHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
