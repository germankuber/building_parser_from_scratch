use std::io;

use crate::parser::Parser;
mod models;
mod parser;
mod tokenizer;
fn main() {
    let mut parser = Parser::new("\"hola\"".to_string());
    let result = parser.parse();
    match result {
        Ok(parsed) => {
            serde_json::to_writer_pretty(io::stdout(), &parsed)
                .expect("Failed to write JSON to stdout");
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
