use lira::{
    Lexer, ProgramParser,
    ir::{
        context::LoweringContext,
        interner::{AsRcRefCell, Interner},
    },
    minifier::minify,
};
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

    let source = if minify {
        concat_dir()
    } else {
        // before changing from String to &str
        // lexer took 88ms to parser 1000x output.li (around 30_000_000 chars/sec)
        fs::read_to_string("output.li")
            .expect("could not read output.li")
    };

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
    let result = parser.parse(lexer).unwrap();

    let interner = Interner::as_rc_refcell();
    let mut lowered = LoweringContext::new(interner);
    let result = lowered.lower_program(result);


    if print {
        println!("{:?}", result);
        // for t in result.statements {
        //     println!("{:?}", t);
        // }
    }

}

fn minifying(lexer: Lexer<'_>) -> () {
    let minified = minify(lexer);

    fs::write("output.li", minified).expect("Failed to write file");
}

fn concat_dir() -> String {
    let mut r = String::new();

    for entry in fs::read_dir("./examples").expect("could not read examples dir") {
        let p = entry.unwrap().path();
        println!("Reading file: {:?}", p);

        r.push_str(&fs::read_to_string(p).expect("could not read to string file"));
        r.push('\n');
    }

    r
}
