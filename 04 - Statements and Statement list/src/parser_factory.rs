use std::collections::{BTreeMap, HashMap};
use indexmap::IndexMap;
use crate::{parser::Parser, tokenizer::TokenType};

pub struct ParserFactory {}

impl ParserFactory {
    pub fn create(to_parse: String) -> Parser {
        let mut spec = IndexMap::new();
        spec.insert(r"^\s+".to_owned(), TokenType::Null);
        spec.insert(r"^\/\*[\s\S]*?\*\/".to_owned(), TokenType::Null);
        spec.insert(r"^;+".to_owned(), TokenType::SemiColon);
        spec.insert(r"^\/\?.*/".to_owned(), TokenType::Null);
        spec.insert(r"^\d+".to_owned(), TokenType::Number);
        spec.insert(r#""([^"]*)"|'([^']*)'"#.to_owned(), TokenType::String);
        // let spec = hashmap![
        //     r"^\s+".to_owned() => TokenType::Null,
        //     r"^\/\*[\s\S]*?\*\/".to_owned() => TokenType::Null,
        //     r"^;+".to_owned() => TokenType::SemiColon,
        //     // Comments
        //     r"^\/\?.*/".to_owned() => TokenType::Null,
        //     // Comments multi line
        //
        //     r"^\d+".to_owned() => TokenType::Number,
        //     r#""([^"]*)"|'([^']*)'"# .to_owned()=> TokenType::String,
        // ];
        Parser::new(spec, to_parse)
    }
}

// Write the test function
#[cfg(test)] // This attribute indicates that the following code is only compiled when running tests
mod tests {
    use serde_json::{json, to_string_pretty, Value};

    // Import the `add` function from the parent module
    use super::*;

    // Define a test function
    #[test]
    fn test_number() {
        let result = ParserFactory::create("42;".to_owned()).parse().unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "type": "Program",
                  "body": [{
                    "type": "ExpressionStatement",
                    "expression": {
                        "type": "NumericLiteral",
                        "value": {
                            "Number": 42
                        }
                    }
                }],
            })
        );
    }

    #[test]
    fn test_string_double_quote() {
        let result = ParserFactory::create(r#" " test : ";"#.to_string())
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "type": "Program",
                "body": [{
                    "type": "ExpressionStatement",
                    "expression": {
                        "type": "StringLiteral",
                        "value": {
                            "String": " test : "
                        }
                    }
                }],
            })
        );
    }

    #[test]
    fn test_string_simple_quote() {
        let result = ParserFactory::create(r#" ' test : ';"#.to_string())
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::from_str::<Value>(&serde_json::to_string(&result).unwrap())
                .unwrap()
                .to_string(),
            json!({
                "type": "Program",
                "body": [{
                    "type": "ExpressionStatement",
                    "expression": {
                        "type": "StringLiteral",
                        "value": {
                            "String": " test : "
                        }
                    }
                }],
            }).to_string()
        );
    }

    #[test]
    fn test_string_and_number() {
        let result = ParserFactory::create(
            r#"
            ' test : ';
               42;"#.to_string(),
        )
            .parse()
            .unwrap();

        let string_json = serde_json::from_str::<Value>(&serde_json::to_string(&result).unwrap())
            .unwrap()
            .to_string();
        let assert_value = &json!({
            "type": "Program",
            "body": [
                {
                    "type": "ExpressionStatement",
                    "expression": {
                        "type": "StringLiteral",
                        "value": {
                            "String": " test : "
                        }
                    }
                },
                {
                    "type": "ExpressionStatement",
                    "expression": {
                        "type": "NumericLiteral",
                        "value": {
                            "Number": 42
                        }
                    }
                }
            ]
        })
            .to_string();
        assert_eq!(&string_json, assert_value);
    }
}
