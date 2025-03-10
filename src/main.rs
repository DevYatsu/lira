use lexer::Token;
use logos::Logos;
use std::fs;

mod lexer;

fn main() {
    let source = fs::read_to_string("examples/one.li")
        .expect("Failed to read file 'examples/one.li'")
        .repeat(1000);

    // let start = std::time::Instant::now();

    let lexer = Token::lexer(&source);

    for result in lexer {
        match result {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(e) => panic!("Lexing error occurred: {:?}", e),
        }
    }
    // let end = std::time::Instant::now();

    println!("Lexing completed successfully",);
}
