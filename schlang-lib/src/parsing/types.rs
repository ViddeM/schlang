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
    Let { id: Identifier, expr: Expression },
    // TEMP for debug purposes
    DebugPrint(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    IntegerLiteral(i64),
    Variable(Identifier),
}
