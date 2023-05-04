use crate::*;
use super::parser::parser;

#[derive(Debug)]
pub enum BinaryOperator {
    Assign,
    Plus,
    Minus,
    Times,
    Div,
    Mod,
    And,
    Or,
    Xor,
    BitAnd,
    BitOr,
    BitXor,
    LShift,
    RShift,
}

impl BinaryOperator {
    pub fn from(op : &Operator) -> BinaryOperator {
        match op {
            Operator::Assign => BinaryOperator::Assign,
            Operator::Plus => BinaryOperator::Plus,
            Operator::Minus => BinaryOperator::Minus,
            Operator::Times => BinaryOperator::Times,
            Operator::Div => BinaryOperator::Div,
            Operator::Mod => BinaryOperator::Mod,
            Operator::And => BinaryOperator::And,
            Operator::Or => BinaryOperator::Or,
            Operator::Xor => BinaryOperator::Xor,
            Operator::BitwiseAnd => BinaryOperator::BitAnd,
            Operator::BitwiseOr => BinaryOperator::BitOr,
            Operator::BitwiseXor => BinaryOperator::BitXor,
            Operator::ShiftLeft => BinaryOperator::LShift,
            Operator::ShiftRight => BinaryOperator::RShift,
            _ => panic!("Invalid operator for BinaryOperator"),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Integer(String),
    Float(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug)]
pub enum OperationResult {
    Literal(Literal),
    Identifier(String),
    BinOpResult(BinaryOperation),
    FuncResult(ApplyFunction),
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operator : BinaryOperator,
    pub left : Box<OperationResult>,
    pub right : Box<OperationResult>,
}

#[derive(Debug)]
pub struct ApplyFunction {
    pub function_id : String,
    pub arguments : Vec<Box<OperationResult>>
}

#[derive(Debug)]
pub struct IfBlock {
    pub condition : OperationResult,
    pub body : Body,
}

#[derive(Debug)]
pub struct ForBlock {
    pub init_statement : OperationResult,
    pub condition : OperationResult,
    pub end_statement : OperationResult,
    pub body : Body,
}

#[derive(Debug)]
pub struct WhileBlock {
    pub condition : OperationResult,
    pub body : Body,
}

#[derive(Debug)]
pub struct FunctionBlock {
    pub identifier : String,
    pub arguments : Vec<Variable>,
    pub return_type : Option<Type>,
    pub body : Body
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub identifier : String,
    pub type_ : Type,
    pub init_value : Option<OperationResult>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration(VariableDeclaration),
    Operation(OperationResult),
    Return(OperationResult),
    If(IfBlock),
    For(ForBlock),
    While(WhileBlock),
    Function(FunctionBlock),
    Struct(std::rc::Rc<CustomType>),
    Body(Body),
}

#[derive(Debug)]
pub struct Body {
    pub statements: Vec<Statement>
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Variable {
    pub identifier : String,
    pub type_ : Type
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct CustomType {
    pub identifier : String,
    pub attributes : Vec<Variable>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum BuiltInType {
    I8,
    I16,
    I32,
    I64,
    ISize,
    U8,
    U16,
    U32,
    U64,
    USize,
    F32,
    F64,
    Bool,
    String
}

impl BuiltInType {
    pub fn from(id : &str) -> Option<BuiltInType> {
        match id {
            "int8" => Some(BuiltInType::I8),
            "int16" => Some(BuiltInType::I16),
            "int32" => Some(BuiltInType::I32),
            "int64" => Some(BuiltInType::I64),
            "uint8" => Some(BuiltInType::U8),
            "uint16" => Some(BuiltInType::U16),
            "uint32" => Some(BuiltInType::U32),
            "uint64" => Some(BuiltInType::U64),
            "fint32" => Some(BuiltInType::F32),
            "fint64" => Some(BuiltInType::F64),
            "isize" => Some(BuiltInType::ISize),
            "usize" => Some(BuiltInType::USize),
            "bool" => Some(BuiltInType::Bool),
            "string" => Some(BuiltInType::String),
            _ => None
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum Type {
    Custom(std::rc::Rc<CustomType>),
    BuiltIn(BuiltInType)
}

#[derive(Debug)]
pub struct TranslationUnit {
    pub custom_types : Vec<std::rc::Rc<CustomType>>,
    pub global_variables : Vec<VariableDeclaration>,
    pub functions : Vec<FunctionBlock>
}

impl TranslationUnit {
    pub fn new() -> TranslationUnit {
        TranslationUnit {
            global_variables : Vec::new(),
            custom_types : Vec::new(),
            functions : Vec::new()
        }
    }

    pub fn from_lexeme(lexeme : &Lexeme) -> TranslationUnit {
        parser(lexeme)
    }
}
