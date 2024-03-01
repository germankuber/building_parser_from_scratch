use std::num::ParseIntError;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomainError {
    ParseIntError,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParsedType {
    NumericLiteral,
    ExpressionStatement,
    StringLiteral,
}
#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub enum ParsedValue {
    String(String),
    Number(u128),
}
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ParsedValueExpression {
    ExpressionStatement(Box<Parsed>),
}

impl Serialize for ParsedValueExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParsedValueExpression::ExpressionStatement(parsed) => parsed.serialize(serializer),
        }
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parsed {
    value: ParsedValue,

    #[serde(rename = "type")]
    parsed_type: ParsedType,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParsedExpression {
    expression: ParsedValueExpression,

    #[serde(rename = "type")]
    parsed_type: ParsedType,
}

impl ParsedExpression {
    pub fn new(expression: ParsedValueExpression) -> ParsedExpression {
        ParsedExpression {
            parsed_type: ParsedType::ExpressionStatement,
            expression,
        }
    }
}
impl Parsed {
    pub fn new(parsed_type: ParsedType, value: ParsedValue) -> Parsed {
        Parsed { parsed_type, value }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum ParsedValues {
    ParsedValue(Parsed),
    #[serde(rename = "expression")]
    ParsedExpression(ParsedExpression),
}
// Implementing custom Serialize for ParsedValues
impl Serialize for ParsedValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParsedValues::ParsedValue(parsed) => parsed.serialize(serializer),
            ParsedValues::ParsedExpression(parsed) => parsed.serialize(serializer),
        }
    }
}

impl From<ParseIntError> for DomainError {
    fn from(_: ParseIntError) -> Self {
        DomainError::ParseIntError
    }
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]

pub struct Program {
    body: Vec<ParsedValues>,

    #[serde(rename = "type")]
    program_type: String,
}

impl Program {
    pub fn new(body: Vec<ParsedValues>) -> Program {
        Program {
            program_type: "Program".to_owned(),
            body,
        }
    }
}
