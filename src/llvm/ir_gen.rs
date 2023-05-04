use std::hash::Hash;
use std::io::Write;
use std::collections::HashMap;
use std::fs::File;

use crate::parser::ast::*;

pub fn create_llvm_ir(tu : &TranslationUnit, file_name : &str) {
    LLVMIRGenerator::new(tu, file_name).generate_ir();
}

/****************************/
/****************************/
/****************************/

type LLVMVariables = HashMap<String, Type>;
type LLVMFunctions = HashMap<String, Option<Type>>;

#[derive(Debug, Hash, PartialEq, Eq)]
struct LLVMFunction {
    name : String,
    return_type : Option<Type>,
}

struct LLVMIRGenerator<'a> {
    tu : &'a TranslationUnit,
    global_variables : LLVMVariables,
    functions : LLVMFunctions,
    output_file : &'a str,
    tmp_variable_count : usize
}

impl<'a> LLVMIRGenerator<'a> {
    fn new(tu : &'a TranslationUnit, output_file : &'a str) -> LLVMIRGenerator<'a> {
        LLVMIRGenerator {
            tu,
            global_variables : LLVMVariables::new(),
            functions : LLVMFunctions::new(),
            output_file,
            tmp_variable_count : 0
        }
    }

    fn generate_ir(&mut self) {
        let mut file = std::fs::File::create(self.output_file).unwrap();

        self.generate_global_variables(&mut file).unwrap_or_else(|e| {
            self.delete_file();
            panic!("Error while generating global variables: {}", e);
        });

        self.generate_functions(&mut file).unwrap_or_else(|e| {
            self.delete_file();
            panic!("Error while generating global variables: {}", e);
        });
    }

    fn delete_file(&self) {
        std::fs::remove_file(self.output_file).unwrap_or_else(|e| {
            panic!("Error while deleting file {}: {}", self.output_file, e);
        });
    }

    fn type_to_llvm_ir(type_ : &Type) -> Result<String, String> {
        match type_ {
            Type::BuiltIn(built_in_type) => {
                match built_in_type {
                    BuiltInType::I8 => Ok("i8".to_string()),
                    BuiltInType::I16 => Ok("i16".to_string()),
                    BuiltInType::I32 => Ok("i32".to_string()),
                    BuiltInType::I64 => Ok("i64".to_string()),
                    BuiltInType::U8 => Ok("i8".to_string()),
                    BuiltInType::U16 => Ok("i16".to_string()),
                    BuiltInType::U32 => Ok("i32".to_string()),
                    BuiltInType::U64 => Ok("i64".to_string()),
                    BuiltInType::F32 => Ok("float".to_string()),
                    BuiltInType::F64 => Ok("double".to_string()),
                    BuiltInType::Bool => Ok("i8".to_string()),
                    _ => Err("Unsupported built-in type!".to_string())
                }
            },
            Type::Custom(_) => {
                panic!("Custom types not supported yet")
            },
        }
    }

    fn operator_to_llvm_ir(op : &BinaryOperator) -> Result<&'static str, String> {
        match op {
            BinaryOperator::Plus => Ok("add"),
            BinaryOperator::Minus => Ok("sub"),
            BinaryOperator::Times => Ok("mul"),
            BinaryOperator::Div => Ok("div"),
            BinaryOperator::Mod => Ok("rem"),
            BinaryOperator::And => Ok("and"),
            BinaryOperator::Or => Ok("or"),
            BinaryOperator::Xor => Ok("xor"),
            _ => Err("Unsupported binary operator".to_string())
        }
    }

    fn generate_global_variables(&mut self, file : &mut File) -> Result<(), String> {
        for var in &self.tu.global_variables {
            if self.global_variables.contains_key(&var.identifier) {
                return Err(format!("Global variable {} already defined", var.identifier));
            }

            let type_llvm = LLVMIRGenerator::type_to_llvm_ir(&var.type_)?;
            let init_value = match &var.init_value {
                None => None,
                Some(OperationResult::Literal(lit)) => match lit {
                    Literal::Integer(i) => Some(i.clone()),
                    Literal::Float(f) => Some(f.clone()),
                    Literal::Boolean(b) => Some(if *b { "1" } else { "0" }.to_string()),
                    Literal::String(_) => panic!("Unsupported literal type: String")
                },
                _ => return Err("Unsupported operation result type".to_string())
            };

            file.write_all(format!(
                "@{} = global {} {}\n",
                var.identifier,
                type_llvm,
                init_value.unwrap_or("0".to_string())
            ).as_bytes()).map_err(|e| e.to_string())?;

            self.global_variables.insert(var.identifier.clone(), var.type_.clone());
        }

        Ok(())
    }

    fn generate_functions(&mut self, file : &mut File) -> Result<(), String> {
        for fun in &self.tu.functions {
            let mut local_variables : LLVMVariables = LLVMVariables::new();
            self.tmp_variable_count = 0;
    
            if self.functions.contains_key(&fun.identifier) {
                return Err(format!("Function {} already defined", fun.identifier));
            }
    
            let return_type_llvm = match &fun.return_type {
                None => "void".to_string(),
                Some(type_) => LLVMIRGenerator::type_to_llvm_ir(type_)?
            };
    
            file.write_all(format!(
                "define {} @{}(",
                return_type_llvm,
                fun.identifier
            ).as_bytes()).map_err(|e| e.to_string())?;
    
            let mut first = true;
            for param in &fun.arguments {
                if !first {
                    file.write_all(", ".as_bytes()).map_err(|e| e.to_string())?;
                }
                first = false;
    
                let type_llvm = LLVMIRGenerator::type_to_llvm_ir(&param.type_)?;
                file.write_all(format!(
                    "{} %{}",
                    type_llvm,
                    param.identifier
                ).as_bytes()).map_err(|e| e.to_string())?;
            }
    
            file.write_all(") {\n".as_bytes()).map_err(|e| e.to_string())?;
    
            for stm in &fun.body.statements {
                match stm {
                    Statement::Declaration(decl) => {
                        let type_llvm = LLVMIRGenerator::type_to_llvm_ir(&decl.type_)?;
    
                        file.write_all(format!(
                            "    %{} = alloca {}\n",
                            decl.identifier,
                            type_llvm
                        ).as_bytes()).map_err(|e| e.to_string())?;
                        
                        local_variables.insert(decl.identifier.clone(), decl.type_.clone());
                        
                        if let None = decl.init_value {
                            continue;
                        };

                        let assign_var = format!("%{}", decl.identifier);

                        self.write_operation(Some(&assign_var), &decl.init_value.as_ref().unwrap(), &local_variables, file)?;
                    },

                    Statement::Operation(op) => {
                        match op {
                            OperationResult::BinOpResult(bin_op) => {
                                match bin_op.operator {
                                    BinaryOperator::Assign => {
                                        let assign_var = match &*bin_op.left {
                                            OperationResult::Identifier(id) => {
                                                id.to_string()
                                            },
                                            _ => return Err("Left hand in assignment must be an identifier".to_string())
                                        };

                                        let assign_var = if local_variables.contains_key(&assign_var) {
                                            format!("%{}", assign_var)
                                        }
                                        else if self.global_variables.contains_key(&assign_var) {
                                            format!("@{}", assign_var)
                                        }
                                        else {
                                            return Err(format!("Variable {} not defined", assign_var));
                                        };

                                        self.write_operation(Some(&assign_var), &bin_op.right, &local_variables, file)?;
                                    },
                                    _ => {
                                        return Err("Unsupported binary operator".to_string());
                                    }
                                }
                            },
                            _ => return Err("Unsupported operation result type".to_string())
                        }
                    },
                    Statement::Return(_ret) => {
                        return Err("Return not supported yet".to_string());
                    },
                    _ => return Err("Unsupported statement type".to_string())
                }
            }

            file.write("}\n\n".as_bytes()).map_err(|e| e.to_string())?;
    
            self.functions.insert(fun.identifier.clone(), fun.return_type.clone());
        }
    
        Ok(())
    }

    fn write_operation(
        &mut self,
        var : Option<&str>,
        op : &OperationResult,
        local_variables : &LLVMVariables,
        file : &mut File
    ) -> Result<Option<usize>, String> {
        let (variable_llvm, ret) = if let Some(var) = var {
            (var.to_string(), None)
        }
        else {
            self.tmp_variable_count += 1;
            (format!("%{}", self.tmp_variable_count-1), Some(self.tmp_variable_count-1))
        };

        match op {
            OperationResult::BinOpResult(bin_op) => {
                let left = self.write_operation(None, &bin_op.left, local_variables, file)?.unwrap();
                let right = self.write_operation(None, &bin_op.right, local_variables, file)?.unwrap();

                let operator = LLVMIRGenerator::operator_to_llvm_ir(&bin_op.operator)?;

                let type_llvm = "i32".to_string(); // TODO: Get type from left and right

                file.write_all(format!(
                    "    {} = {} {} %{}, %{}\n",
                    variable_llvm,
                    operator,
                    type_llvm,
                    left,
                    right
                ).as_bytes()).map_err(|e| e.to_string())?;

                self.tmp_variable_count += 1;
            },
            OperationResult::Literal(lit) => {
                let (lit_val, type_llvm) = match lit {
                    Literal::Integer(v) => (v.clone(),"i32"),
                    Literal::Float(v) => (v.clone(),"double"),
                    Literal::Boolean(v) => (if *v {"1".to_string()} else {"0".to_string()} ,"i8"),
                    Literal::String(_) => return Err("Unsupported literal type: String".to_string())
                };

                file.write_all(format!(
                    "    {} = {} {}\n",
                    variable_llvm,
                    type_llvm,
                    lit_val
                ).as_bytes()).map_err(|e| e.to_string())?;
            },
            OperationResult::Identifier(id) => {
                let id = if local_variables.contains_key(id) {
                    format!("%{}", id)
                }
                else if self.global_variables.contains_key(id) {
                    format!("@{}", id)
                }
                else {
                    return Err(format!("Variable {} not defined", id));
                };

                file.write_all(format!(
                    "    {} = load {}\n",
                    variable_llvm,
                    id
                ).as_bytes()).map_err(|e| e.to_string())?;
            },
            _ => return Err("Unsupported operation result type".to_string())

        }
        Ok(ret)
    }

    
}
