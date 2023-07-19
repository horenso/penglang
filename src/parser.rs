use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::{delimited, preceded};
use nom::IResult;

// E -> T {+|- T}*
// T -> F {*|/ F}*
// F -> num
//    | (E)

#[derive(Debug, PartialEq)]
pub enum Expression {
    Number(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
}

// pub fn parse_factor(input: &str) -> IResult<&str, Expression> {
//     alt((parse_number, parse_parenthesized_expression));
// }

// pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
//     parse_term(input)
// }

// fn parse_term(input: &str) -> IResult<&str, Expression> {
//     let parse_factor =

//     let parse_mul = map(preceded(char('*'), parse_factor), |expr| {
//         Expression::Mul(Box::new(Expression::Number(0)), Box::new(expr))
//     });
//     let parse_div = map(preceded(char('/'), parse_factor), |expr| {
//         Expression::Div(Box::new(Expression::Number(0)), Box::new(expr))
//     });

//     parse_factor(input).and_then(|(rest, init_expr)| {
//         alt((
//             map(parse_mul, |expr| {
//                 Expression::Mul(Box::new(init_expr.clone()), Box::new(expr))
//             }),
//             map(parse_div, |expr| {
//                 Expression::Div(Box::new(init_expr.clone()), Box::new(expr))
//             }),
//             |input| Ok((input, init_expr)),
//         ))(rest)
//     })
// }

// fn parse_number(input: &str) -> IResult<&str, Expression> {
//     let (rest, token) = nom::character::complete::digit1(input)?;
//     let num = token.parse::<i64>().unwrap();
//     Ok((rest, Expression::Number(num)))
// }

// fn parse_parenthesized_expression(input: &str) -> IResult<&str, Expression> {
//     let (rest, expression) = delimited(char('('), parse_expression)(input)?;
//     let (rest, _) = char(')')(rest)?;
//     Ok((rest, expression))
// }
