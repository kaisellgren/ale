use lexical_analysis::tokens;
use lexical_analysis::tokens::Token;
use lexical_analysis::tokens::PunctuationKind;
use lexical_analysis::tokens::WhitespaceKind;
use lexical_analysis::tokens::SpecialKind;

#[derive(Debug, Clone)]
pub struct Type(String);

#[derive(Debug, Clone)]
pub struct Symbol(String);

#[derive(Debug)]
pub enum AST {
    Program(Vec<AST>),
    Statement(Statement),
    Expression(Expression)
}

#[derive(Debug)]
pub enum Statement {
    TypeSignature { parameter_types: Vec<Type>, return_type: Type },
    FunctionDefinition {
        identifier: Symbol,
        parameters: Vec<Symbol>,
        body: Expression
    }
}

#[derive(Debug)]
pub enum Expression {
    FunctionCall {
        identifier: Symbol,
        parameters: Vec<Expression>
    },
    Literal(LiteralKind),
    Identifier(Symbol)
}

#[derive(Debug)]
pub enum LiteralKind {
    StringLiteral(String),
    IntegerLiteral(String),
    DecimalLiteral(String),
    BooleanLiteral(String)
}

pub fn parse(tokens: &[Token]) -> AST {
    let mut statements = Vec::new();
    let mut position = 0usize;

    while position < tokens.len() {
        match tokens.get(position).unwrap() {
            &Token::Special(SpecialKind::SignatureStart) => {
                // TODO: position_elem()
                let sub_tokens: Vec<Token> = tokens.iter().skip(position).take_while(|t| **t != Token::Whitespace(WhitespaceKind::NewLine)).cloned().collect();
                position += sub_tokens.len();
                statements.push(parse_type_signature(sub_tokens.as_ref()))
            },
            &Token::Identifier(_) => {
                // TODO: position_elem()
                let sub_tokens: Vec<Token> = tokens.iter().skip(position).take_while(|t| **t != Token::Whitespace(WhitespaceKind::NewLine)).cloned().collect();
                position += sub_tokens.len();
                statements.push(parse_function_definition(sub_tokens.as_ref()))
            },
            ref token if should_ignore(&token) => position += 1,
            token => panic!("Unexpected token at root: {:?}", token)
        }
    }

    AST::Program(statements)
}

fn parse_function_definition(tokens: &[Token]) -> AST {
    let mut symbols = Vec::new();
    let mut position = 0usize;

    while position < tokens.len() {
        match tokens[position].clone() {
            Token::Identifier(name) => {
                position += 1;
                symbols.push(Symbol(name))
            },
            Token::Punctuation(PunctuationKind::Equal) => {
                position += 1;
                break
            },
            token => panic!("Unexpected token at function definition: {:?}", token)
        }
    }

    let body = parse_expression(&tokens[position..]);

    AST::Statement(Statement::FunctionDefinition {
        identifier: symbols.first().expect("Expected a function definition to have a name").clone(),
        parameters: symbols[1..symbols.len()].to_vec(),
        body: body
    })
}

fn parse_expression(tokens: &[Token]) -> Expression {
    // x + y + z    (x + y) + z
    // x + y
    // print (x + 5)

    // + x y
    // print (+ x 5)
    if tokens.len() > 1 {
        match tokens.first().unwrap().clone() {
            Token::Identifier(function) => {
                let parameters = tokens[1 .. tokens.len()].to_vec();
                Expression::FunctionCall {
                    identifier: Symbol(function),
                    parameters: parameters.iter()
                        .map(|p| parse_expression(&[p.clone()]))
                        .collect()
                }
            },
            token => panic!("Unexpected token in expression parsing: {:?}", token)
        }
    } else {
        match tokens.first() {
            Some(&Token::Identifier(ref name)) => Expression::Identifier(Symbol(name.clone())),
            Some(&Token::Literal(ref kind, ref name)) => kind.to_expression(name.clone()),
            None => panic!("Expected a token in expression parsing"),
            token => panic!("Unexpected token in expression parsing: {:?}", token)
        }
    }
}

fn parse_type_signature(tokens: &[Token]) -> AST {
    let mut types = Vec::new();
    let mut position = 0usize;

    while position < tokens.len() {
        match tokens[position].clone() {
            Token::Special(SpecialKind::SignatureArrow) | Token::Special(SpecialKind::SignatureStart) => position += 1,
            Token::Identifier(name) => {
                position += 1;
                types.push(Type(name))
            },
            Token::Whitespace(_) | Token::Comment(_) => break,
            token => panic!("Unexpected token at type signature: {:?}", token)
        }
    }

    AST::Statement(Statement::TypeSignature {
        parameter_types: types[0 .. types.len()].to_vec(),
        return_type: types.last().expect("Expected type identifier(s) after a type signature").clone()
    })
}

fn should_ignore(t: &Token) -> bool {
    match t {
        &Token::Comment(_) => true,
        &Token::Whitespace(_) => true,
        _ => false
    }
}

impl tokens::LiteralKind {
    fn to_expression(&self, name: String) -> Expression {
        Expression::Literal(
            match self {
                &tokens::LiteralKind::StringLiteral => LiteralKind::StringLiteral(name),
                &tokens::LiteralKind::IntegerLiteral => LiteralKind::IntegerLiteral(name),
                &tokens::LiteralKind::DecimalLiteral => LiteralKind::DecimalLiteral(name),
                &tokens::LiteralKind::BooleanLiteral => LiteralKind::BooleanLiteral(name)
            }
        )
    }
}
