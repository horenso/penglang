use queues::{IsQueue, Queue};

use crate::{expression::Expression, lexer::Token};

pub fn parse(tokens: Queue<Token>) -> Result<Expression, String> {
    let mut parser = Parser::new(tokens);
    parser.parse_expression()
}

struct Parser {
    tokens: Queue<Token>,
}

impl Parser {
    pub fn new(tokens: Queue<Token>) -> Parser {
        Parser { tokens }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.peek().ok()
    }

    fn eat(&mut self) -> Option<Token> {
        self.tokens.remove().ok()
    }

    // E -> T {'+' | '-' T}*
    fn parse_expression(&mut self) -> Result<Expression, String> {
        let mut term = self.parse_term()?;
        while let Option::Some(next) = self.peek() {
            match next {
                Token::Add => {
                    self.eat();
                    term = Expression::Add(Box::new(term), Box::new(self.parse_term()?))
                }
                Token::Sub => {
                    self.eat();
                    term = Expression::Sub(Box::new(term), Box::new(self.parse_term()?))
                }
                _ => break,
            }
        }
        Ok(term)
    }

    // T -> F {'*' | '/' F}*
    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut factor = self.parse_factor()?;
        while let Option::Some(next) = self.peek() {
            match next {
                Token::Mul => {
                    self.eat();
                    factor = Expression::Mul(Box::new(factor), Box::new(self.parse_factor()?))
                }
                Token::Div => {
                    self.eat();
                    factor = Expression::Div(Box::new(factor), Box::new(self.parse_factor()?))
                }
                _ => break,
            }
        }
        Ok(factor)
    }

    // F -> A {'**' A}*
    // ** is right-associative
    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut factor = self.parse_atom()?;
        while let Option::Some(next) = self.peek() {
            if next == Token::Power {
                self.eat();
                let exponent = self.parse_factor()?;
                factor = Expression::Power(Box::new(factor), Box::new(exponent));
            } else {
                break;
            }
        }
        Ok(factor)
    }

    // A -> Token::Int | (E)
    fn parse_atom(&mut self) -> Result<Expression, String> {
        if let Some(next) = self.eat() {
            match next {
                Token::LParen => {
                    let expression = self.parse_expression()?;
                    match self.eat() {
                        Some(Token::RParen) => Ok(expression),
                        _ => Err(String::from("Expected closing ')'")),
                    }
                }
                Token::Int(val) => Ok(Expression::Int(val.clone())),
                _ => Err(String::from("Expected '(' or int")),
            }
        } else {
            Err(String::from("Empty Expression"))
        }
    }
}
