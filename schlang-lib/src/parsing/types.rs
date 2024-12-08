use crate::common_types::Identifier;

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
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
    // TEMP for debug purposes
    DebugPrint(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    Variable(Identifier),
}
