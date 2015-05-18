#![allow(dead_code)]
extern crate regex;

mod lexical_analysis;
mod syntax_analysis;

use lexical_analysis::lex;
use syntax_analysis::parse;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut data = String::new();
    let mut f = File::open("./src/test").unwrap();
    f.read_to_string(&mut data).unwrap();

    let tokens = lex(data.as_ref());
    println!("LEXER: \n-----\n{:?}\n-----\n", tokens);

    let ast = parse(tokens.as_ref());
    println!("AST: \n-----\n{:?}\n-----\n", ast);
}
