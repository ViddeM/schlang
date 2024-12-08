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
    Let {
        id: Identifier,
        expr: Expression,
    },
    If {
        cond: Expression,
        block: Box<Statement>,
    },
    IfElse {
        cond: Expression,
        if_block: Box<Statement>,
        else_block: Box<Statement>,
    },
    IfElseIf {
        if_cond: Expression,
        if_block: Box<Statement>,
        else_if_cond: Expression,
        else_if_block: Box<Statement>,
    },
    IfElseIfElse {
        if_cond: Expression,
        if_block: Box<Statement>,
        else_if_cond: Expression,
        else_if_block: Box<Statement>,
        else_block: Box<Statement>,
    },
    Block(Vec<Statement>),
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
