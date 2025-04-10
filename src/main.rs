use lira::{Lexer, ProgramParser, minifier::minify};
use std::fs;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let print = args
        .get(1)
        .map(|x| x == "p" || x == "print")
        .unwrap_or(false);
    let minify = args
        .get(1)
        .map(|x| x == "m" || x == "minify")
        .unwrap_or(false);

    let source = fs::read_to_string("examples/fn.li")
        .expect("Failed to read file")
        .repeat(1);

    let char_count = source.len();

    let start = std::time::Instant::now();

    let lexer = Lexer::new(&source);

    if print {
        for t in lexer.clone() {
            println!("{:?}", t);
        }
    }

    if minify {
        minifying(lexer);
    } else {
        parsing(lexer, print);
    }

    let duration = start.elapsed();
    let millis = duration.as_secs_f64() * 1000.0;
    let chars_per_sec = char_count as f64 / duration.as_secs_f64();

    println!("{} characters processed in {:.2} ms", char_count, millis);
    println!("Throughput: {:.2} chars/sec", chars_per_sec)
}

fn parsing(lexer: Lexer<'_>, print: bool) -> () {
    let parser = ProgramParser::new();
    let result = parser.parse(lexer);

    if let Ok(result) = result {
        if print {
            println!("{:?}", result);
            // for t in result.statements {
            //     println!("{:?}", t);
            // }
        }
    } else {
        println!("{:?}", result);
    }
}

fn minifying(lexer: Lexer<'_>) -> () {
    let minified = minify(lexer);

    fs::write("output.li", minified).expect("Failed to write file");
}
