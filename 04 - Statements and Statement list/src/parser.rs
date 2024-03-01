use std::collections::{BTreeMap, HashMap};
use indexmap::IndexMap;

use crate::{
    models::{
        DomainError, Parsed, ParsedExpression, ParsedType, ParsedValue, ParsedValueExpression,
        ParsedValues, Program,
    },
    tokenizer::{TokenType, TokenValue, Tokenizer},
};

pub struct Parser {
    tokenizer: Tokenizer,
    look_ahead: Option<TokenValue>,
}
impl Parser {
    pub fn new(spec: IndexMap<String, TokenType>, to_parse: String) -> Parser {
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
        self.statement_list().map(|x| Program::new(x))
    }
    // StatementList
    // : Statement
    // | StatementList Statement -> Statement Statement Statement Statement
    // ;

    pub fn statement_list(&mut self) -> Result<Vec<ParsedValues>, DomainError> {
        let mut statement_list = vec![self.statement()?];
        while self.look_ahead.is_some() {
            statement_list.push(self.statement()?);
        }
        Ok(statement_list)
    }

    // Statement
    // : ExpressionStatement
    // ;
    pub fn statement(&mut self) -> Result<ParsedValues, DomainError> {
        self.expression_statement()
    }

    // ExpressionStatement
    // : Expression ;
    // ;
    pub fn expression_statement(&mut self) -> Result<ParsedValues, DomainError> {
        let expression = self.expression()?;
        let token = self.eat(TokenType::SemiColon);
        let aa: ParsedValueExpression =
            ParsedValueExpression::ExpressionStatement(Box::new(expression));
        let expression_parsed = ParsedExpression::new(aa);
        let response = ParsedValues::ParsedExpression(expression_parsed);
        Ok(response)
    }
    pub fn expression(&mut self) -> Result<Parsed, DomainError> {
        self.literal()
    }

    pub fn literal(&mut self) -> Result<Parsed, DomainError> {
        match self.look_ahead.as_ref().unwrap().token_type {
            TokenType::Number => self.numeric_literal(),
            TokenType::String => self.string_literal(),
            TokenType::Null => todo!(),
            TokenType::SemiColon => todo!(),
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
