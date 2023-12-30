use super::lexeme::Range;

#[derive(Debug)]
pub enum LexerError {
    CouldNotReadLine(usize),
    CouldNotOpenFile(String),
    TokenError(Range),
}

impl LexerError {
    pub fn get_msg(&self) -> String {
        match self {
            Self::CouldNotReadLine(n_line) => format!("Could not read line {}", n_line),
            Self::CouldNotOpenFile(f) => format!("Could not open file {}", f),
            Self::TokenError(r) => format!("Error reading token at {}:{}-{}", r.line+1, r.char_begin+1, r.char_end),
        }
    }
}
