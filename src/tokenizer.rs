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
}

impl Tokenizer {
    pub fn new(to_parse: String) -> Tokenizer {
        Tokenizer {
            to_parse,
            cursor: 0,
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
        if let Some(matched) = Regex::new(r"^\d+").unwrap().find(&string_data) {
            self.cursor += matched.as_str().len();
            return Some(TokenValue {
                token_type: TokenType::Number,
                value: ParsedValue::Number(matched.as_str().parse::<u128>().unwrap()),
            });
        }
        if let Some(matched) = Regex::new(r#""([^"]*)""#).unwrap().find(&string_data) {
            let value_data = matched.as_str();
            self.cursor += value_data.len();
            return Some(TokenValue {
                token_type: TokenType::String,
                value: ParsedValue::String(value_data.to_string()),
            });
        }
        None
    }
}
