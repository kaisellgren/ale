pub mod tokens;

use regex::Regex;
use lexical_analysis::tokens::Token;
use lexical_analysis::tokens::Token::*;
use lexical_analysis::tokens::PunctuationKind;
use lexical_analysis::tokens::PunctuationKind::*;
use lexical_analysis::tokens::SpecialKind;
use lexical_analysis::tokens::SpecialKind::*;

pub struct Lexer<'a> {
    position: usize,
    data: &'a str,
    tokens: Vec<Token>
}

impl<'a> Lexer<'a> {
    fn process_next(&mut self) {
        match self.peek().as_ref() {
            ";" => {
                let comment: String = self.data
                    .chars()
                    .skip(self.position)
                    .take_while(|c| !is_line_ending(c))
                    .collect();

                self.position += 1 + comment.len();
                self.tokens.push(Token::Comment(comment));
            },
            "(" => self.push_punctuation(LeftParen),
            ")" => self.push_punctuation(RightParen),
            "[" => self.push_punctuation(LeftBracket),
            "]" => self.push_punctuation(RightBracket),
            "{" => self.push_punctuation(LeftBrace),
            "}" => self.push_punctuation(RightBrace),
            ":" if self.peek_forward(1) == ":" => {
                //self.push_punctuation();
                self.position += 1;
            },
            "=" if self.peek_forward(1) != "=" => self.push_special(FunctionDefinition),
            c if is_identifier(c) => {
                let name: String = self.data
                    .chars()
                    .skip(self.position)
                    .take_while(|c| is_identifier(c.to_string().as_ref()))
                    .collect();

                self.position += name.len();
                self.tokens.push(Identifier(name));
            },
            c if should_ignore(c) => self.position += 1,
            c => panic!("Unexpected character: {}", c)
        };
    }

    fn push_punctuation(&mut self, token: PunctuationKind) {
        self.push(Punctuation(token));
    }

    fn push_special(&mut self, token: SpecialKind) {
        self.push(Special(token));
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
        self.position += 1;
    }

    fn peek(&self) -> String {
        self.data.chars().skip(self.position).take(1).collect()
    }

    fn peek_forward(&self, n: usize) -> String {
        self.data.chars().skip(self.position + n).take(1).collect()
    }
}

pub fn lex(data: &str) -> Vec<Token> {
    let mut lexer = Lexer {
        data: data,
        position: 0,
        tokens: Vec::new()
    };

    while lexer.position < lexer.data.len() {
        lexer.process_next();
    }

    lexer.tokens
}

fn should_ignore(c: &str) -> bool {
    match c {
        "\r" | "\n" | " " | "\t" => true,
        _ => false
    }
}

fn is_line_ending(c: &char) -> bool {
    match *c {
        '\r' | '\n' => true,
        _ => false
    }
}

fn is_identifier(c: &str) -> bool {
    let r = Regex::new(r"[a-zA-Z0-9_!#%&?/\\@|<>.*^~=+:'-]").unwrap();
    r.is_match(c)
}
