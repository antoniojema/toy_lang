use crate::*;
use super::ast::*;

const PRIORITY : &'static [&'static [Operator]] = &[
    &[Operator::Comma],
    &[Operator::Assign, Operator::PlusEq, Operator::MinusEq, Operator::TimesEq, Operator::DivEq, Operator::ModEq, Operator::AndEq, Operator::OrEq, Operator::XorEq, Operator::ShiftLeftEq, Operator::ShiftRightEq, Operator::BitwiseAndEq, Operator::BitwiseOrEq, Operator::BitwiseXorEq],
    &[Operator::Plus, Operator::Minus],
    &[Operator::Times, Operator::Div, Operator::Mod],
    &[Operator::Equals, Operator::NotEquals],
    &[Operator::Or],
    &[Operator::And],
    &[Operator::Xor],
    &[Operator::LessThan, Operator::LessEqThan, Operator::GreaterThan, Operator::GreaterEqThan],
    &[Operator::BitwiseOr],
    &[Operator::BitwiseAnd],
    &[Operator::BitwiseXor],
    &[Operator::ShiftLeft, Operator::ShiftRight],
    &[Operator::Point]
];

pub struct OperationParser<'a> {
    lexeme: &'a Lexeme,
    token_begin : usize,
    token_end : usize
}

impl<'a> OperationParser<'a> {

pub fn new(lexeme : &'a Lexeme, token_begin : usize, token_end : usize) -> OperationParser<'a> {
    OperationParser{
        lexeme,
        token_begin,
        token_end
    }
}

pub fn parse(&mut self) -> OperationResult {
    return self.read_operation_in(self.token_begin, self.token_end);
}

fn find_any_token_in(&self, tok_begin : usize, tok_end : usize, tokens : &[Token], reverse : bool) -> Option<usize> {
    let range = tok_begin..tok_end;
    let delim_open : Token;
    let delim_close : Token;
    let range : Vec<usize> =
        if reverse {
            delim_open  = Token::Delimiter(Delimiter::ParenthesisClose);
            delim_close = Token::Delimiter(Delimiter::ParenthesisOpen);
            range.rev().collect()
        } else {
            delim_open  = Token::Delimiter(Delimiter::ParenthesisOpen);
            delim_close = Token::Delimiter(Delimiter::ParenthesisClose);
            range.collect()
        };
    
    let mut n_parenthesis = 0;
    for i in range {
        let token_at_i = &self.lexeme.token_at(i).token;

        if *token_at_i == delim_open {
            n_parenthesis += 1;
        }
        else if *token_at_i == delim_close {
            n_parenthesis -= 1;
        }

        if n_parenthesis > 0 {
            continue;
        }
        
        for token in tokens.iter() {
            if *token_at_i == *token {
                return Some(i);
            }
        }
    }
    None
}

fn find_any_operator_in(&self, tok_begin : usize, tok_end : usize, operators : &[Operator], reverse : bool) -> Option<usize> {
    let tokens : Vec<Token> = operators
        .iter()
        .map(|op| Token::Operator(*op))
        .collect();

    self.find_any_token_in(tok_begin, tok_end, &tokens, reverse)
}

fn find_highest_priority_op_in(&mut self, tok_begin : usize, tok_end : usize) -> Option<usize> {
    for operators in PRIORITY.iter() {
        if let Some(index) = self.find_any_operator_in(tok_begin, tok_end, operators, true) {
            return Some(index);
        }
    }
    None
}

fn read_operations_sep_by_commas_in(&mut self, tok_begin : usize, tok_end : usize) -> Vec<Box<OperationResult>> {
    let mut operations : Vec<Box<OperationResult>> = Vec::new();
    let mut n_parenthesis = 0;
    let mut last_comma = tok_begin;
    for i in tok_begin..tok_end {
        let token_at_i = &self.lexeme.token_at(i).token;

        if let Token::Delimiter(Delimiter::ParenthesisOpen) = token_at_i {
            n_parenthesis += 1;
        }
        else if let Token::Delimiter(Delimiter::ParenthesisClose) = token_at_i {
            n_parenthesis -= 1;
        }

        if n_parenthesis > 0 {
            continue;
        }

        if let Token::Operator(Operator::Comma) = token_at_i {
            operations.push(Box::new(self.read_operation_in(last_comma, i)));
            last_comma = i+1;
        }
    }
    operations.push(Box::new(self.read_operation_in(last_comma, tok_end)));

    operations
}

fn read_operation_in(&mut self, tok_begin : usize, tok_end : usize) -> OperationResult {
    println!("Reading operation in [{:#?}, {:#?})", self.lexeme.token_at(tok_begin), self.lexeme.token_at(tok_end));

    // Read highest priority operator
    if let Some(i) = self.find_highest_priority_op_in(tok_begin, tok_end) {
        let op = match &self.lexeme.token_at(i).token {
            Token::Operator(op) => op,
            _ => panic!("This should never happen")
        };

        return OperationResult::BinOpResult(BinaryOperation{
            left  : Box::new(self.read_operation_in(tok_begin, i)),
            right : Box::new(self.read_operation_in(i+1, tok_end)),
            operator : BinaryOperator::from(&op)
        });
    }

    // Read function call
    if tok_end - tok_begin >= 3 {
        let tok_id : &Token = &self.lexeme.token_at(tok_begin).token;
        let tok_par_open : &Token = &self.lexeme.token_at(tok_begin+1).token;
        let tok_par_close : &Token = &self.lexeme.token_at(tok_end-1).token;

        if let Token::Identifier(id) = tok_id {
        if let Token::Delimiter(Delimiter::ParenthesisOpen) = tok_par_open {
        if let Token::Delimiter(Delimiter::ParenthesisClose) = tok_par_close {
            return OperationResult::FuncResult(ApplyFunction{
                function_id : id.clone(),
                arguments : self.read_operations_sep_by_commas_in(tok_begin+2, tok_end-1)
            });
        }}}
    }
    let token_at_begin = &self.lexeme.token_at(tok_begin).token;

    // Read parenthesis
    let token_at_end = &self.lexeme.token_at(tok_end-1).token;

    if let Token::Delimiter(Delimiter::ParenthesisOpen) = token_at_begin {
        if let Token::Delimiter(Delimiter::ParenthesisClose) = token_at_end {
            return self.read_operation_in(tok_begin+1, tok_end-1);
        }
    }

    // Read declaration

    // Read identifier or literal;
    if tok_end == tok_begin+1 {
        let token_at_begin = &self.lexeme.token_at(tok_begin).token;
        if let Token::Identifier(id) = token_at_begin {
            return OperationResult::Identifier(id.clone());
        }
        else if let Token::Integer(str) = token_at_begin {
            return OperationResult::Literal(Literal::Integer(str.clone()));
        }
        else if let Token::Float(str) = token_at_begin {
            return OperationResult::Literal(Literal::Float(str.clone()));
        }
        else if let Token::String(str) = token_at_begin {
            return OperationResult::Literal(Literal::String(str.clone()));
        }
        else if let Token::Boolean(b) = token_at_begin {
            return OperationResult::Literal(Literal::Boolean(b.clone()));
        }
    }

    panic!("Could not read operation");
}

}