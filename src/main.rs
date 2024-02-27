#[macro_use]
extern crate maplit;

use std::io;

use tokenizer::TokenType;

use crate::parser::Parser;
mod models;
mod parser;
mod tokenizer;
fn main() {
    let spec = hashmap![
        r"^\s+".to_owned() => TokenType::Null,
        // Comments
        r"^\/\?.*/".to_owned() => TokenType::Null,
        // Comments multi line
        r"^\/\*[\s\S]*?\*\/".to_owned() => TokenType::Null,
        r"^\d+".to_owned() => TokenType::Number,
        r#""([^"]*)"|'([^']*)'"# .to_owned()=> TokenType::String,
    ];
    let mut parser = Parser::new(
        spec,
        r#"   
        //comments
        /**
         * comments
         */
     "  hola    "  
      "#
        .to_string(),
    );
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
