use lexical_analysis::tokens::Token::*;
use lexical_analysis::tokens::LiteralKind::*;
use lexical_analysis::tokens::SpecialKind::*;
use lexical_analysis::tokens::WhitespaceKind::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Punctuation(PunctuationKind),
    Comment(String),
    Literal(LiteralKind, String),
    Special(SpecialKind),
    Whitespace(WhitespaceKind)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralKind {
    StringLiteral,
    IntegerLiteral,
    DecimalLiteral,
    BooleanLiteral
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PunctuationKind {
    Comma,
    Equal,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialKind {
    FunctionDefinition,
    SignatureStart,
    SignatureArrow
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhitespaceKind {
    NewLine,
    Indentation
}

/// Retrieves the full size of the token in the original input stream.
pub fn token_size(token: &Token) -> usize {
    match token {
        &Identifier(ref s) => s.len(),
        &Keyword(ref s) => s.len(),
        &Comment(ref s) => s.len() + 1,
        &Punctuation(_) => 1,
        &Literal(ref kind, ref s) => match kind {
            &StringLiteral => s.len() + 2,
            _ => s.len()
        },
        &Special(ref kind) => match kind {
            &FunctionDefinition => 1,
            &SignatureStart => 2,
            &SignatureArrow => 2
        },
        &Whitespace(ref kind) => match kind {
            &NewLine => 1,
            &Indentation => 4
        }
    }
}
