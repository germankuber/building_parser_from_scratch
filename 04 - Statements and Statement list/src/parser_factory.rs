use crate::{parser::Parser, tokenizer::TokenType};

pub struct ParserFactory {}
impl ParserFactory {
    pub fn create(to_parse: String) -> Parser {
        let spec = hashmap![
            r"^\s+".to_owned() => TokenType::Null,
            // Comments
            r"^\/\?.*/".to_owned() => TokenType::Null,
            // Comments multi line
            r"^\/\*[\s\S]*?\*\/".to_owned() => TokenType::Null,
            r"^\d+".to_owned() => TokenType::Number,
            r#""([^"]*)"|'([^']*)'"# .to_owned()=> TokenType::String,
        ];
        Parser::new(spec, to_parse)
    }
}
// Write the test function
#[cfg(test)] // This attribute indicates that the following code is only compiled when running tests
mod tests {
    use serde_json::{json, to_string_pretty};

    // Import the `add` function from the parent module
    use super::*;

    // Define a test function
    #[test]
    fn test_number() {
        let result = ParserFactory::create("42".to_owned()).parse().unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "type": "Program",
                "body": {
                    "type": "NumericLiteral",
                    "value": {
                        "Number": 42
                    }
                }
            })
        );
    }
    #[test]
    fn test_string_double_quote() {
        let result = ParserFactory::create(r#" " test : ""#.to_string())
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "type": "Program",
                "body": {
                    "type": "StringLiteral",
                    "value": {
                        "String":" test : "
                    }
                }
            })
        );
    }
    #[test]
    fn test_string_simple_quote() {
        let result = ParserFactory::create(r#" ' test : '"#.to_string())
            .parse()
            .unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({
                "type": "Program",
                "body": {
                    "type": "StringLiteral",
                    "value": {
                        "String":" test : "
                    }
                }
            })
        );
    }
}
