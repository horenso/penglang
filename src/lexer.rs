use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{char, i64, multispace0};
use nom::combinator::{map, opt, value};
use nom::sequence::{delimited, preceded};
use nom::IResult;
use queues::{IsQueue, Queue};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Int(i64),
    String(String),
    Add,
    Sub,
    Mul,
    Div,
    Power,
    LParen,
    RParen,
}

pub fn lex(input: &str) -> Result<Queue<Token>, String> {
    let mut tokens = Queue::new();
    let mut remaining_input = input;

    while !remaining_input.is_empty() {
        match lex_next(remaining_input) {
            IResult::Ok((new_remaining_input, token)) => {
                tokens.add(token).unwrap();
                remaining_input = new_remaining_input;
            }
            IResult::Err(error) => return Err(error.to_string()),
        }
    }

    Ok(tokens)
}

fn lex_next(input: &str) -> IResult<&str, Token> {
    let parse_add = map(char('+'), |_| Token::Add);
    let parse_sub = map(char('-'), |_| Token::Sub);
    let parse_mul = map(char('*'), |_| Token::Mul);
    let parse_div = map(char('/'), |_| Token::Div);
    let parse_power = map(tag("**"), |_| Token::Power);
    let parse_lparen = map(char('('), |_| Token::LParen);
    let parse_rparen = map(char(')'), |_| Token::RParen);

    preceded(
        parse_ignored,
        alt((
            parse_add,
            parse_sub,
            parse_power,
            parse_mul,
            parse_div,
            parse_lparen,
            parse_rparen,
            parse_int_literal,
            parse_string_literal,
        )),
    )(input)
}

fn parse_ignored(input: &str) -> IResult<&str, ()> {
    let parse_comment = delimited(tag("#"), is_not("\n"), opt(char('\n')));
    let parse_ignored = alt((multispace0, parse_comment));
    value((), parse_ignored)(input)
}

fn parse_int_literal(input: &str) -> IResult<&str, Token> {
    let result = i64(input)?;
    Ok((result.0, Token::Int(result.1)))
}

fn parse_string_literal(input: &str) -> IResult<&str, Token> {
    delimited(
        char('"'),
        map(take_until("\""), |str: &str| Token::String(str.to_owned())),
        char('"'),
    )(input)
}

#[cfg(test)]
mod tests {
    use queues::queue;

    use super::*;

    #[test]
    fn test_lexer() {
        let mut parsed = lex(" ( 1 + 2 ) \"wow\" ").unwrap();
        let mut expected = queue![
            Token::LParen,
            Token::Int(1),
            Token::Add,
            Token::Int(2),
            Token::RParen,
            Token::String("wow".to_owned())
        ];
        while parsed.size() > 0 || expected.size() > 0 {
            assert_eq!(parsed.remove().unwrap(), expected.remove().unwrap());
        }
    }
}
