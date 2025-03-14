use lexer::Lexer;
use parser::ProgramParser;
use std::fs;

pub(crate) mod ast;
pub(crate) mod lexer;
mod parser;

fn main() {
    let source = fs::read_to_string("examples/let.li")
        .expect("Failed to read file")
        .repeat(1);

    let start = std::time::Instant::now();

    let lexer = Lexer::new(&source);

    for t in lexer.clone() {
        println!("{:?}", t);
    }

    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    println!("{:?}", result);

    // for result in lexer {
    //     match result {
    //         Ok(token) => {
    //             println!("{:?}", token);
    //         }
    //         Err(e) => panic!("Lexing error occurred: {:?}", e),
    //     }
    // }

    let end = std::time::Instant::now();

    println!("Lexing completed successfully in {}ms",(end-start).as_millis());
}
