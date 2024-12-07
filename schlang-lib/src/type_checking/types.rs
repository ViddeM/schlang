use crate::common_types::{Identifier, Type};

#[derive(Debug, Clone)]
pub struct Program {
    pub main_function: Function,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Identifier,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let { id: Identifier, expr: Expression },
    DebugPrint(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    Variable(Identifier, Type),
}

impl Expression {
    pub fn get_type(&self) -> Type {
        match self {
            Expression::IntegerLiteral(_) => Type::Integer,
            Expression::Variable(_, t) => t.clone(),
        }
    }
}
