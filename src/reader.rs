use std::{fs::File, io::Read};

use crate::parser::Parser;

pub fn read_file(path: String) {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Unable to read the file");

    let mut parser = Parser::new(content);
    parser.scan_tokens();
    if !parser.errors.is_empty() {
        parser.report();
    } else {
        dbg!(parser.tokens);
    }
    // println!("Content:");
    // println!("{}", content);
    // println!("Not yet implemented");
}
