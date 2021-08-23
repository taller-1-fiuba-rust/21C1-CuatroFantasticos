use crate::client::Client;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::server;
use std::ops::Add;

/// CommandHandler
///
/// Takes a command and executes it (parses it too)
///
pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle(request: &HttpRequest) -> HttpResponse {
        let redis_request =
            server::handler::command::CommandHandler::parse_command(request.msg_body.clone());
        let mut client = Client::new().expect("Could not create a client");
        let response = client.execute_command(redis_request);
        HttpResponse::new("200", None, Some(response))
    }
}

impl CommandHandler {
    pub fn parse_command(body: String) -> String {
        let body = body.trim_matches(char::from(0)).trim();
        let qty_args = body.split(' ').count();
        let mut result = format!("*{}\r\n", qty_args);

        let args = body.split(' ');
        for arg in args {
            result = result.add(&format!("${}\r\n{}\r\n", arg.len(), arg));
        }
        result = result.add("\r\n");
        result
    }
}
