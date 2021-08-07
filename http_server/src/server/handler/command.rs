use crate::client::Client;
use crate::http::request::HttpRequest;
use crate::http::response::HttpResponse;
use crate::server;
use std::ops::Add;

/// Ejecuta un comando
pub struct CommandHandler {}

impl CommandHandler {
    pub fn handle(request: &HttpRequest) -> HttpResponse {
        let redis_request =
            server::handler::command::CommandHandler::parse_command(request.msg_body.clone());
        let mut client = Client::new().expect("could not create a client");
        let response = client.execute_command(redis_request);
        server::handler::command::CommandHandler::format_response(response)
    }
}

impl CommandHandler {
    pub fn parse_command(body: String) -> String {
        let args = body.split(' ');
        let qty_args = args.count();

        //esto esta feo, pero sino me dice que se mueve el args

        let args = body.split(' ');
        let mut result = format!("*{}\r\n", qty_args);
        for arg in args {
            result = result.add(&format!("${}\r\n", arg));
        }
        result = result.add("\r\n");
        result
    }

    pub fn format_response(_response: String) -> HttpResponse<'static> {
        todo!()
    }
}
