use std::collections::HashMap;

use crate::{
    models::{DomainError, Parsed, ParsedType, ParsedValue, Program},
    tokenizer::{TokenType, TokenValue, Tokenizer},
};

pub struct Parser {
    tokenizer: Tokenizer,
    look_ahead: Option<TokenValue>,
}
impl Parser {
    pub fn new(spec: HashMap<String, TokenType>, to_parse: String) -> Parser {
        Parser {
            tokenizer: Tokenizer::new(spec, to_parse.clone()),
            look_ahead: None,
        }
    }
    pub fn parse(&mut self) -> Result<Program, DomainError> {
        self.look_ahead = self.tokenizer.get_next_token();
        self.program()
    }

    pub fn program(&mut self) -> Result<Program, DomainError> {
        self.literal().map(|x| Program::new(x))
    }

    pub fn literal(&mut self) -> Result<Parsed, DomainError> {
        match self.look_ahead.as_ref().unwrap().token_type {
            TokenType::Number => self.numeric_literal(),
            TokenType::String => self.string_literal(),
            TokenType::Null => todo!(),
        }
    }
    // NumericLiteral
    // : Number
    // ;
    fn numeric_literal(&mut self) -> Result<Parsed, DomainError> {
        let token = self.eat(TokenType::Number);
        Ok(Parsed::new(
            ParsedType::NumericLiteral,
            ParsedValue::Number(token.value.get_number()),
        ))
    }
    fn string_literal(&mut self) -> Result<Parsed, DomainError> {
        let token = self.eat(TokenType::String);
        let value = token.value.get_string();
        return Ok(Parsed::new(
            ParsedType::StringLiteral,
            ParsedValue::String(value[1..value.len() - 1].to_string()),
        ));
    }
    fn eat(&mut self, token_type: TokenType) -> TokenValue {
        if self.look_ahead.is_none() {
            panic!("Unexpected end of input");
        }
        let token = self.look_ahead.clone().unwrap();
        if token.token_type != token_type {
            panic!("Unexpected token {:?}", token);
        }
        self.look_ahead = self.tokenizer.get_next_token();
        token
    }
}
