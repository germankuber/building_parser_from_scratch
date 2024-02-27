use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::models::ParsedValue;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenType {
    Number,
    String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TokenValue {
    pub token_type: TokenType,
    pub value: ParsedValue,
}
pub struct Tokenizer {
    to_parse: String,
    cursor: usize,
    spec: HashMap<String, TokenType>,
}

impl Tokenizer {
    pub fn new(spec: HashMap<String, TokenType>, to_parse: String) -> Tokenizer {
        Tokenizer {
            to_parse,
            cursor: 0,
            spec,
        }
    }
    fn has_more_tokens(&self) -> bool {
        self.cursor < self.to_parse.len()
    }
    pub fn get_next_token(&mut self) -> Option<TokenValue> {
        if !self.has_more_tokens() {
            return None;
        }
        let string_data = self.to_parse[self.cursor..].to_string();
        if let Some(token) = self.search_token(&string_data) {
            self.cursor += token.0.len();
            return Some(TokenValue {
                token_type: token.1,
                value: self.parse(&token.0),
            });
        }
        None
    }
    fn search_token(&self, string_data: &str) -> Option<(String, TokenType)> {
        for (key, value) in self.spec.iter() {
            if let Some(token) = self.match_token(key, &string_data) {
                return Some((token, value.clone()));
            }
        }
        None
    }
    fn match_token(&self, regex: &str, data_to_parse: &str) -> Option<String> {
        if let Some(matched) = Regex::new(regex).unwrap().find(data_to_parse) {
            return Some(matched.as_str().to_string());
        }
        None
    }
    fn parse(&mut self, value: &str) -> ParsedValue {
        if let Ok(parsed) = value.parse::<u128>() {
            return ParsedValue::Number(parsed);
        }
        ParsedValue::String(value.to_string())
    }
}
