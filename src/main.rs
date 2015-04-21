#![allow(dead_code)]
extern crate regex;

mod lexical_analysis;

use lexical_analysis::lex;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new();
    let mut f = File::open("./src/test").unwrap();
    f.read_to_string(&mut data).unwrap();

    println!("Lexer results: \n{:?}", lex(data.as_ref()));
}
