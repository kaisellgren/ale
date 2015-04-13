#![allow(dead_code)]
extern crate regex;

mod lexical_analysis;

use lexical_analysis::Lexer;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Lexing...");

    let mut data = String::new();
    let mut f = File::open("./src/test").unwrap();
    f.read_to_string(&mut data).unwrap();

    let mut lexer = Lexer::new(data.as_ref());
    println!("Lexer results: \n{:?}", lexer.lex());
}
