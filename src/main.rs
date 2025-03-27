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
        .repeat(1000);

    let char_count = source.len();

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

        let duration = start.elapsed();
        let millis = duration.as_secs_f64() * 1000.0;
        let chars_per_sec = char_count as f64 / duration.as_secs_f64();

        println!(
            "{} characters lexed and parse in {:.2} ms",
            char_count, millis
        );
        println!("Throughput: {:.2} chars/sec", chars_per_sec)
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
