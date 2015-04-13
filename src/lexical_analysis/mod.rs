use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Special(SpecialKind),
    Comment(String),
    Literal(LiteralKind)
}

#[derive(Debug)]
pub enum LiteralKind {
    String,
    Integer,
    Decimal,
    Boolean
}

#[derive(Debug)]
pub enum SpecialKind {
    Comma,
    FunctionDefinition,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace
}

pub struct Lexer<'a> {
    position: usize,
    data: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Lexer {
        Lexer {
            data: data,
            position: 0
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.data.len() {
            self.process_next(&mut tokens);
        }

        tokens
    }

    fn process_next(&mut self, tokens: &mut Vec<Token>) {
        match self.peek().as_ref() {
            ";" => {
                let comment: String = self.data
                    .chars()
                    .skip(self.position)
                    .take_while(|c| !is_line_ending(c))
                    .collect();

                self.position += 1 + comment.len();
                tokens.push(Token::Comment(comment));
            },
            c if is_identifier_start(c) => {
                let name: String = self.data
                    .chars()
                    .skip(self.position)
                    .take_while(is_identifier_after)
                    .collect();

                self.position += name.len();
                tokens.push(Token::Identifier(name));
            },
            "=" => {
                tokens.push(Token::Special(SpecialKind::FunctionDefinition));
                self.position += 1;
            },
            
            c if should_ignore(c) => self.position += 1,
            c => panic!("Unexpected character: {}", c)
        }
    }

    fn peek(&self) -> String {
        self.data.chars().skip(self.position).take(1).collect()
    }
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

fn is_identifier_start(c: &str) -> bool {
    let r = Regex::new(r"[a-zA-Z]").unwrap();
    r.is_match(c)
}

fn is_identifier_after(c: &char) -> bool {
    let r = Regex::new(r"[a-zA-Z0-9_]").unwrap();
    r.is_match(c.to_string().as_ref())
}
