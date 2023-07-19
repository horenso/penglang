use penglang::lexer::lex;
use std::io::{self, BufRead, BufReader};

fn main() {
    let reader = BufReader::new(io::stdin());

    for line in reader.lines() {
        println!("Parsing result: {:?}", lex(line.unwrap().as_str()));
    }
}
