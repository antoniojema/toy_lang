use super::tokens::Token;
use super::lexer::lexer;

#[derive(Debug)]
pub struct Range {
    pub line : usize,
    pub char_begin : usize,
    pub char_end : usize,
}

#[derive(Debug)]
pub struct ContextedToken {
    pub token : Token,
    pub range : Range,
}

#[derive(Debug)]
pub struct Lexeme {
    tokens : Vec<ContextedToken>
}

impl Lexeme {
    pub fn new () -> Lexeme {
        Lexeme{tokens : vec![]}
    }

    pub fn with_capacity (capacity : usize) -> Lexeme {
        Lexeme{tokens : Vec::with_capacity(capacity)}
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn push(&mut self, token : ContextedToken) {
        self.tokens.push(token);
    }

    pub fn token_at(&self, index : usize) -> &ContextedToken {
        &self.tokens[index]
    }

    pub fn mut_token_at(&mut self, index : usize) -> &mut ContextedToken {
        &mut self.tokens[index]
    }

    pub fn from_file(file : &str) -> Lexeme {
        lexer(file)
    }
}