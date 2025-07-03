use lox_lang::{parser::Parser, reader};
use std::{
    cmp::Ordering,
    io::{self, BufRead, Write},
    process::exit,
};

fn main() {
    let args = std::env::args();
    match args.len().cmp(&2) {
        Ordering::Less => read_prompt(),
        Ordering::Equal => reader::read_file(args.last().unwrap()),
        Ordering::Greater => {
            eprintln!("Usage: jlox [script]");
            exit(64);
        }
    }
}

fn read_prompt() {
    let mut reader = std::io::BufReader::new(std::io::stdin());
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut buff = String::new();
        reader.read_line(&mut buff).unwrap();
        if buff.is_empty() {
            break;
        }
        run(buff);
    }
}

fn run(source: String) {
    let mut parser = Parser::new(source);
    parser.scan_tokens();
    dbg!(&parser.errors);

    // if parser.errors.len() > 0 {
    //     parser.report();
    // } else {
    //     dbg!(parser.tokens);
    // }
    // let tokens = source.split_ascii_whitespace();
    // for token in tokens {
    //     println!("Token: {token}");
    // }
}
