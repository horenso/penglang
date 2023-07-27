use penglang::{expression::Expression, lexer::lex, parser::parse};
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), String> {
    let reader = BufReader::new(io::stdin());

    for line in reader.lines() {
        let tokens = lex(line.unwrap().as_str()).map_err(|e| format!("Lexing error: {}", e))?;
        println!("Tokens: {:?}", tokens);
        let expression = parse(tokens).map_err(|e| format!("Parsing error: {}", e))?;
        println!("Expression tree: {:?}", expression);
        println!(
            "Evaluation: {:?}",
            Expression::eval(expression).map_err(|e| format!("Evaluation error: {}", e))?
        );
    }

    Ok(())
}
