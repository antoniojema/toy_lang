#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Def,
    If,
    Else,
    For,
    While,
    Return,
    Class,
    Void
}

impl Keyword {
    pub fn to_string<'a,'b>(&'a self) -> &'b str {
        match self {
            Keyword::Def => return "def",
            Keyword::If => return "if",
            Keyword::Else => return "else",
            Keyword::For => return "for",
            Keyword::While => return "while",
            Keyword::Return => return "return",
            Keyword::Class => return "class",
            Keyword::Void => return "void",
        }
    }

    pub fn from_string(s : &str) -> Option<Keyword> {
        match s {
            "def" => return Some(Keyword::Def),
            "if" => return Some(Keyword::If),
            "else" => return Some(Keyword::Else),
            "for" => return Some(Keyword::For),
            "while" => return Some(Keyword::While),
            "return" => return Some(Keyword::Return),
            "class" => return Some(Keyword::Class),
            "void" => return Some(Keyword::Void),
            _ => None,
        }
    }

    pub fn len(&self) -> usize {
        return self.to_string().len();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator {
    Assign,

    Plus,
    Minus,
    Times,
    Div,
    Mod,

    PlusEq,
    MinusEq,
    TimesEq,
    DivEq,
    ModEq,

    Equals,
    NotEquals,
    LessThan,
    LessEqThan,
    GreaterThan,
    GreaterEqThan,

    ShiftLeft,
    ShiftRight,

    ShiftLeftEq,
    ShiftRightEq,

    Not,

    And,
    Or,
    Xor,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,

    AndEq,
    OrEq,
    XorEq,
    BitwiseAndEq,
    BitwiseOrEq,
    BitwiseXorEq,

    Comma,
    Point,
}

impl Operator {
    pub fn to_string<'a,'b>(&'a self) -> &'b str {
        match self {
            Operator::Assign => return "=",

            Operator::Plus => return "+",
            Operator::Minus => return "-",
            Operator::Times => return "*",
            Operator::Div => return "/",
            Operator::Mod => return "%",

            Operator::PlusEq => return "+=",
            Operator::MinusEq => return "-=",
            Operator::TimesEq => return "*=",
            Operator::DivEq => return "/=",
            Operator::ModEq => return "%=",

            Operator::Equals => return "==",
            Operator::NotEquals => return "!=",
            Operator::LessThan => return "<",
            Operator::LessEqThan => return "<=",
            Operator::GreaterThan => return ">",
            Operator::GreaterEqThan => return ">=",

            Operator::ShiftLeft => return "<<",
            Operator::ShiftRight => return ">>",

            Operator::ShiftLeftEq => return "<<=",
            Operator::ShiftRightEq => return ">>=",

            Operator::Not => return "!",

            Operator::And => return "&&",
            Operator::Or => return "||",
            Operator::Xor => return "^^",
            Operator::BitwiseAnd => return "&",
            Operator::BitwiseOr => return "|",
            Operator::BitwiseXor => return "^",

            Operator::AndEq => return "&&=",
            Operator::OrEq => return "||&",
            Operator::XorEq => return "^^&",
            Operator::BitwiseAndEq => return "&=",
            Operator::BitwiseOrEq => return "|=",
            Operator::BitwiseXorEq => return "^=",

            Operator::Comma => return ",",
            Operator::Point => return ".",
        }
    }

    pub fn from_string(s : &str) -> Option<Operator> {
        match s {
            "=" => return Some(Operator::Assign),
            "+" => return Some(Operator::Plus),
            "-" => return Some(Operator::Minus),
            "*" => return Some(Operator::Times),
            "/" => return Some(Operator::Div),
            "%" => return Some(Operator::Mod),
            "+=" => return Some(Operator::PlusEq),
            "-=" => return Some(Operator::MinusEq),
            "*=" => return Some(Operator::TimesEq),
            "/=" => return Some(Operator::DivEq),
            "%=" => return Some(Operator::ModEq),
            "==" => return Some(Operator::Equals),
            "!=" => return Some(Operator::NotEquals),
            "<" => return Some(Operator::LessThan),
            "<=" => return Some(Operator::LessEqThan),
            ">" => return Some(Operator::GreaterThan),
            ">=" => return Some(Operator::GreaterEqThan),
            "<<" => return Some(Operator::ShiftLeft),
            ">>" => return Some(Operator::ShiftRight),
            "<<=" => return Some(Operator::ShiftLeftEq),
            ">>=" => return Some(Operator::ShiftRightEq),
            "!" => return Some(Operator::Not),
            "&&" => return Some(Operator::And),
            "||" => return Some(Operator::Or),
            "^^" => return Some(Operator::Xor),
            "&" => return Some(Operator::BitwiseAnd),
            "|" => return Some(Operator::BitwiseOr),
            "^" => return Some(Operator::BitwiseXor),
            "&&=" => return Some(Operator::AndEq),
            "||&" => return Some(Operator::OrEq),
            "^^&" => return Some(Operator::XorEq),
            "&=" => return Some(Operator::BitwiseAndEq),
            "|=" => return Some(Operator::BitwiseOrEq),
            "^=" => return Some(Operator::BitwiseXorEq),
            "," => return Some(Operator::Comma),
            "." => return Some(Operator::Point),
            _ => return None,
        }
    }

    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Delimiter {
    ParenthesisOpen,
    ParenthesisClose,
    BracketsOpen,
    BracketsClose,
    SquareBracketsOpen,
    SquareBracketsClose,
}

impl Delimiter {
    pub fn len(&self) -> usize {
        return 1;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    EndOfFile,
    EndOfStatement,
    Keyword(Keyword),
    Operator(Operator),
    Integer(String),
    Float(String),
    String(String),
    Boolean(bool),
    Identifier(String),
    Delimiter(Delimiter),
    Comment(String)
}

impl Token {
    pub fn len(&self) -> usize {
        match self {
            Token::EndOfFile => return 0,
            Token::EndOfStatement => return 1,
            Token::Keyword(kw) => return kw.len(),
            Token::Operator(op) => return op.len(),
            Token::Integer(s) => return s.len(),
            Token::Float(s) => return s.len(),
            Token::String(s) => return s.len()+2,
            Token::Boolean(_) => return 1,
            Token::Identifier(s) => return s.len(),
            Token::Delimiter(d) => return d.len(),
            Token::Comment(s) => return s.len()+2,
        }
    }
}