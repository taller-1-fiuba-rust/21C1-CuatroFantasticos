static TOKEN_SEPARATOR: &str = "\r\n";

struct Parser {}
// Simple implementation of parser for our TP
impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    fn parse_bulk_len(command_part: &str) -> Result<int, String> {
        if &command_part[..1] != "*" {
            Err("Not a bulk len token")
        }
        match &command_part[..1] {
            "*" => Ok(&command_part[1..]),
            _ => println!("Other"),
        };
    }

    pub fn parse(&self, packed_command: &[u8]) -> Self {
        let packed_command = std::str::from_utf8(packed_command)?.split(TOKEN_SEPARATOR);
        bulk_len = packed_command.next();

        for command_part in packed_command {}
    }
}
