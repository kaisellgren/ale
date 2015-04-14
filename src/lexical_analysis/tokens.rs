#[derive(Debug)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Punctuation(PunctuationKind),
    Comment(String),
    Literal(LiteralKind),
    Special(SpecialKind)
}

#[derive(Debug)]
pub enum LiteralKind {
    String,
    Integer,
    Decimal,
    Boolean
}

#[derive(Debug)]
pub enum PunctuationKind {
    Comma,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace
}

#[derive(Debug)]
pub enum SpecialKind {
    FunctionDefinition,
    SignatureStart,
    SignatureArrow
}
