use super::lexeme::*;
use super::tokens::*;
use std::fs;
use std::io::{BufReader, BufRead};

fn open_file(file_path : &str) -> fs::File {
    match fs::File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}",e);
        } 
    }
}

fn is_identifier_init(ch : char) -> bool {
    return
        ch.is_alphabetic() ||
        ch == '_' ||
        ch == '$';
}

fn is_identifier_continuation(ch: char) -> bool {
    return
        is_identifier_init(ch) ||
        ch.is_numeric();
}

fn is_number_init(ch : char) -> bool {
    return
        ch.is_numeric() ||
        ch == '.';
}

fn find_token(line : &str) -> Token {
    let mut chars = line.chars();
    let char_0 = chars.next().unwrap();
    let char_1 = chars.next();
    let char_2 = chars.next();

    match char_0 {
        ';' => {return Token::EndOfStatement},
        
        '(' => {return Token::Delimiter(Delimiter::ParenthesisOpen)},
        ')' => {return Token::Delimiter(Delimiter::ParenthesisClose)},
        '{' => {return Token::Delimiter(Delimiter::BracketsOpen)},
        '}' => {return Token::Delimiter(Delimiter::BracketsClose)},
        '[' => {return Token::Delimiter(Delimiter::SquareBracketsOpen)},
        ']' => {return Token::Delimiter(Delimiter::SquareBracketsClose)},
        ',' => {return Token::Operator(Operator::Comma)},

        '+' => match char_1 {
            Some('=') => {return Token::Operator(Operator::PlusEq)}
            _ => {return Token::Operator(Operator::Plus)}
        },

        '-' => match char_1 {
            Some('=') => {return Token::Operator(Operator::MinusEq)}
            _ => {return Token::Operator(Operator::Minus)}
        },

        '*' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::TimesEq)},
            _ => {return Token::Operator(Operator::Times)}
        }},

        '/' => {match char_1 {
            Some('/') => {return Token::Comment(String::from(&line[2..]))},
            Some('=') => {return Token::Operator(Operator::DivEq)},
            _ => {return Token::Operator(Operator::Div)}
        }},

        '%' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::ModEq)},
            _ => {return Token::Operator(Operator::Mod)}
        }},

        '=' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::Equals)},
            _ => {return Token::Operator(Operator::Assign)}
        }},

        '!' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::NotEquals)},
            _ => {return Token::Operator(Operator::Not)}
        }},

        '<' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::LessEqThan)},
            Some('<') => match char_2 {
                Some('=') => {return Token::Operator(Operator::ShiftLeftEq)},
                _ => {return Token::Operator(Operator::ShiftLeft)}
            },
            _ => {return Token::Operator(Operator::LessThan)}
        }},

        '>' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::GreaterEqThan)},
            Some('>') => match char_2 {
                Some('=') => {return Token::Operator(Operator::ShiftRightEq)},
                _ => {return Token::Operator(Operator::ShiftRight)}
            },
            _ => {return Token::Operator(Operator::GreaterThan)}
        }},

        '&' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::BitwiseAndEq)},
            Some('&') => match char_2 {
                Some('=') => {return Token::Operator(Operator::AndEq)},
                _ => {return Token::Operator(Operator::And)}
            },
            _ => {return Token::Operator(Operator::BitwiseAnd)}
        }},

        '|' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::BitwiseOrEq)},
            Some('|') => match char_2 {
                Some('=') => {return Token::Operator(Operator::OrEq)},
                _ => {return Token::Operator(Operator::Or)}
            },
            _ => {return Token::Operator(Operator::BitwiseOr)}
        }},

        '^' => {match char_1 {
            Some('=') => {return Token::Operator(Operator::BitwiseXorEq)},
            Some('^') => match char_2 {
                Some('=') => {return Token::Operator(Operator::XorEq)},
                _ => {return Token::Operator(Operator::Xor)}
            },
            _ => {return Token::Operator(Operator::BitwiseXor)}
        }},

        '.' => {match char_1 {
            Some(c) if c.is_numeric() => {},
            _ => {return Token::Operator(Operator::Point)}
        }},
        
        _ => {}
    }
    
    if is_identifier_init(char_0) {
        let mut n : usize = 0;
        for ch in line.chars() {
            if !is_identifier_continuation(ch) {break}
            n += 1;
        }

        let s = line[..n].trim();

        match Keyword::from_string(s) {
            Some(kw) => return Token::Keyword(kw),
            None => {},
        }

        return Token::Identifier(String::from(s));
    }

    if is_number_init(char_0) {
        let mut point_found : bool = false;
        let mut e_found : bool = false;
        let mut e_just_found : bool = false;
        let mut e_pm_just_found : bool = false;

        let mut n : usize = 0;
        for ch in line.chars() {
            if !point_found && ch == '.' {
                point_found = true;
                n += 1;
                continue;
            }
            if !e_found && (ch == 'e' || ch == 'E') {
                e_found = true;
                e_just_found = true;
                n += 1;
                continue;
            }
            if e_just_found {
                e_just_found = false;
                if ch == '+' || ch == '-' {
                    e_pm_just_found = true;
                    n += 1;
                    continue;
                }
                if !ch.is_numeric() {
                    panic!("Invalid numeric literal: {}", line);
                }
            }
            if e_pm_just_found {
                e_pm_just_found = false;
                if !ch.is_numeric() {
                    panic!("Invalid numeric literal: {}", line);
                }
            }

            if !ch.is_numeric() {break};
            n += 1;
        }

        let s = line[..n].trim();
        if point_found || e_found {
            return Token::Float(String::from(s));
        } else {
            return Token::Integer(String::from(s));
        };
    }

    if char_0 == '"' {
        let mut n : usize = 0;
        for ch in line[1..].chars() {
            if ch == '"' {break}
            n += 1;
        }

        let s = &line[1..n+1];
        return Token::String(String::from(s));
    }

    panic!("Invalid character found: {}", char_0);
}

fn find_first_not_of(s : &str, chars : Vec<char>) -> Option<usize> {
    let mut n : usize = 0;
    for ch in s.chars() {
        let mut some_equal = false;
        for ch2 in chars.iter() {
            if ch == *ch2 {
                some_equal = true;
                break;
            }
        }

        if !some_equal {return Some(n);}
        
        n += 1;
    }
    return None;
}

fn read_tokens_in_line<'a,'b>(lexeme: &'a mut Lexeme, mut line : &'b str, n_line : usize) {
    let mut ch : usize = match find_first_not_of(line, vec![' ', '\n', '\r', '\t']) {
        Some(n) => n,
        None => return,
    };

    line = &line[ch..];

    while line.len() > 0 {
        let token = find_token(&line);
        let token_len = token.len();
        
        match token {
            Token::Comment(_) => {},
            _ => {
                lexeme.push(ContextedToken{
                    token : token,
                    range : Range {
                        line : n_line,
                        char_begin : ch,
                        char_end : ch+token_len,
                    }
                });
            }
        }
        
        line = &line[token_len..];
        
        let spaces_len = match find_first_not_of(line, vec![' ', '\n', '\r', '\t']) {
            Some(n) => n,
            None => return,
        };
        line = &line[spaces_len..];
        
        ch += token_len + spaces_len;
        
    }
}

pub fn lexer(file_path : &str) -> Lexeme {
    let mut lexeme = Lexeme::new();

    let file : fs::File = open_file(file_path);
    let reader = BufReader::new(file);

    for (n_line, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        read_tokens_in_line(&mut lexeme, &line, n_line);
    }

    return lexeme;
}
