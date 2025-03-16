use lexer::Lexer;
use parser::ProgramParser;
use std::fs;

pub(crate) mod ast;
pub(crate) mod lexer;
mod parser;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let no_print = args.get(1).is_some();

    let source = fs::read_to_string("examples/test.li")
        .expect("Failed to read file")
        .repeat(10000);

    let length = source.len();

    let start = std::time::Instant::now();

    let lexer = Lexer::new(&source);

    if no_print {
        for t in lexer.clone() {
            println!("{:?}", t);
        }
    }

    let parser = ProgramParser::new();

    let result = parser.parse(lexer);

    if let Ok(result) = result {
        if no_print {
            for t in result.statements {
                println!("{:?}", t);
            }
        }

        let end = std::time::Instant::now();

        println!(
            "{} characters lexed and parse in {}ms",
            length,
            (end - start).as_millis()
        );
    } else {
        println!("{:?}", result);
    }

    // for result in lexer {
    //     match result {
    //         Ok(token) => {
    //             println!("{:?}", token);
    //         }
    //         Err(e) => panic!("Lexing error occurred: {:?}", e),
    //     }
    // }
}
