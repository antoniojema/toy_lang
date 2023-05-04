use crate::{*, lexer::lexeme::ContextedToken};
use super::ast::*;
use super::operation_parser::*;

pub fn parser(lexeme : &Lexeme) -> TranslationUnit {
    let mut ast_builder = TUBuilder::new(lexeme);
    ast_builder.build();
    ast_builder.tu
}

/*******************/
/*******************/
/*******************/

struct TUBuilder<'a> {
    lexeme: &'a Lexeme,
    current_token : usize,
    tu : TranslationUnit,
}

impl<'a> TUBuilder<'a> {
    fn new(lexeme : &Lexeme) -> TUBuilder {
        TUBuilder{
            lexeme,
            current_token : 0,
            tu : TranslationUnit::new(),
        }
    }

    fn get_custom_type_from_id(&self, id : &str) -> Option<&std::rc::Rc<CustomType>> {
        self.tu.custom_types
            .iter()
            .find(|ct| ct.identifier == id)
    }

    fn get_type_from_id(&self, id : &str) -> Option<Type> {
        if let Some(type_) = BuiltInType::from(id) {
            return Some(Type::BuiltIn(type_));
        }

        if let Some(type_) = self.get_custom_type_from_id(id) {
            return Some(Type::Custom(type_.clone()));
        }

        return None;
    }

    fn last_token(&self) -> &ContextedToken {
        self.lexeme.token_at(self.current_token)
    }

    fn next_token(&mut self) -> &ContextedToken {
        self.current_token += 1;
        self.lexeme.token_at(self.current_token)
    }

    fn incr_token(&mut self) {
        self.current_token += 1;
    }

    fn build(&mut self) {
        while self.current_token < self.lexeme.len() {
            match self.read_statement() {
                Some(Statement::Declaration(dec)) => self.tu.global_variables.push(dec),
                Some(Statement::Function(fun)) => self.tu.functions.push(fun),
                Some(Statement::Struct(type_)) => self.tu.custom_types.push(type_),
                _ => panic!("Unexpected statement"),
            }
        }
    }

    fn read_statement(&mut self) -> Option<Statement> {
        while let Token::EndOfStatement = self.last_token().token {
            self.incr_token();
        }

        match self.last_token().token {
            Token::EndOfStatement => None,
            Token::EndOfFile => None,
            Token::Delimiter(Delimiter::BracketsOpen) => Some(Statement::Body(self.read_body())),
            Token::Keyword(Keyword::Def) => Some(Statement::Function(self.read_function())),
            // Token::Keyword(Keyword::Return) => Some(Statement::Return(self.read_return())),
            // Token::Keyword(Keyword::If) => Some(Statement::If(self.read_if())),
            // Token::Keyword(Keyword::For) => Some(Statement::For(self.read_for())),
            // Token::Keyword(Keyword::While) => Some(Statement::While(self.read_while())),
            _ => {
                if let Token::Identifier(id) = &self.last_token().token {
                    if let Some(_) = self.get_type_from_id(id) {
                        return Some(Statement::Declaration(self.read_declaration()));
                    }
                }

                return Some(Statement::Operation(self.read_operation()))
            }
        }
    }

    fn read_body(&mut self) -> Body {
        let mut statements : Vec<Statement> = vec![];
        loop {
            match self.next_token().token {
                Token::Delimiter(Delimiter::BracketsClose) => break,
                _ => {},
            };
            if let Some(s) = self.read_statement() {
                statements.push(s);
            }
        }

        self.incr_token();

        Body {
            statements
        }
    }

    fn read_function(&mut self) -> FunctionBlock {
        // Read type
        self.incr_token();
        let ret_id_tok = self.last_token();
        let return_type : Option<Type> = match &ret_id_tok.token {
            Token::Keyword(Keyword::Void) => None,
            Token::Identifier(id) => Some(
                self
                    .get_type_from_id(id)
                    .unwrap_or_else(|| panic!("Unknown return type"))
            ),
            _ => panic!("Expected function identifier"),
        };

        // Read identifier
        let fun_id_tok = self.next_token();
        let fun_id : String = match &fun_id_tok.token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected function identifier"),
        };

        let token = self.next_token();
        match token.token {
            Token::Delimiter(Delimiter::ParenthesisOpen) => {},
            _ => panic!("Expected argument list"),
        };

        // Read argument list
        let mut arguments : Vec<Variable> = vec![];

        loop {
            let type_id: String = match &self.next_token().token {
                Token::Identifier(id) => id.clone(),
                Token::Delimiter(Delimiter::ParenthesisClose) => break,
                _ => panic!("Expected identifier or end of argument list"),
            };
            let arg_id: String = match &self.next_token().token {
                Token::Identifier(id) => id.clone(),
                _ => panic!("Expected identifier"),
            };

            let type_ = self.get_type_from_id(&type_id).unwrap_or_else(|| panic!("Unknown type"));

            arguments.push(Variable {
                identifier: arg_id,
                type_
            });
        }

        self.incr_token();

        let body = self.read_body();

        FunctionBlock {
            identifier : fun_id,
            arguments,
            return_type,
            body,
        }
    }

    fn find_end_of_statement(&mut self) -> Option<usize> {
        for n_tok in self.current_token..self.lexeme.len() {
            if let Token::EndOfStatement = self.lexeme.token_at(n_tok).token {
                return Some(n_tok);
            }
        }
        None
    }

    fn read_operation(&mut self) -> OperationResult {
        let tok_begin : usize = self.current_token;
        let tok_end : usize = self.find_end_of_statement().unwrap_or_else(|| panic!("Did not find end of statement"));

        self.current_token = tok_end;

        OperationParser::new(self.lexeme, tok_begin, tok_end).parse()
    }

    fn read_declaration(&mut self) -> VariableDeclaration {
        let type_ = match &self.last_token().token {
            Token::Identifier(id) => self.get_type_from_id(id).unwrap(),
            _ => panic!("Expected identifier"),
        };

        let var_id = match &self.next_token().token {
            Token::Identifier(id) => id.clone(),
            _ => panic!("Expected identifier"),
        };

        let init_value = match &self.next_token().token {
            Token::EndOfStatement => {None},
            Token::Operator(Operator::Assign) => {self.incr_token();  Some(self.read_operation())},
            _ => panic!("Expected semicolon or assignment"),
        };

        VariableDeclaration {
            identifier: var_id,
            type_,
            init_value
        }
    }
}
