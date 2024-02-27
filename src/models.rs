use std::num::ParseIntError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomainError {
    ParseIntError,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedType {
    NumericLiteral,
    StringLiteral,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedValue {
    String(String),
    Number(u128),
}

impl ParsedValue {
    pub fn get_number(&self) -> u128 {
        match self {
            ParsedValue::Number(n) => *n,
            _ => panic!("Not a number"),
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            ParsedValue::String(s) => s.clone(),
            _ => panic!("Not a string"),
        }
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Parsed {
    #[serde(rename = "type")]
    parsed_type: ParsedType,
    value: ParsedValue,
}

impl Parsed {
    pub fn new(parsed_type: ParsedType, value: ParsedValue) -> Parsed {
        Parsed { parsed_type, value }
    }
}

impl From<ParseIntError> for DomainError {
    fn from(_: ParseIntError) -> Self {
        DomainError::ParseIntError
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]

pub struct Program {
    #[serde(rename = "type")]
    program_type: String,
    body: Parsed,
}

impl Program {
    pub fn new(body: Parsed) -> Program {
        Program {
            program_type: "Program".to_owned(),
            body,
        }
    }
}
