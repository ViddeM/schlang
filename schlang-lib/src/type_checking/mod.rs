use std::collections::HashMap;

use types::{Expression, Function, Program, Statement};

use crate::{
    common_types::{Identifier, Type},
    parsing::types::{
        Expression as ParsedExpression, Program as ParsedProgram, Statement as ParsedStatement,
    },
};

pub mod types;

#[derive(thiserror::Error, Debug)]
pub enum TypeCheckError {
    #[error("No `main` function found")]
    MissingMainFunction,
    #[error("Variable `{0}` not found in scope")]
    VariableNotFound(Identifier),
    #[error("Variable `{0}` already exists in scope")]
    VariableAlreadyExists(Identifier),
}

pub type TypeCheckResult<T> = Result<T, TypeCheckError>;

pub fn type_check(program: ParsedProgram) -> TypeCheckResult<Program> {
    let main = program
        .functions
        .into_iter()
        .find(|f| f.name.0.as_str() == "main")
        .ok_or(TypeCheckError::MissingMainFunction)?;

    let mut variables: HashMap<Identifier, Type> = HashMap::new();

    let mut typed_stmts = vec![];

    for stmt in main.statements.into_iter() {
        let stmt = match stmt {
            ParsedStatement::Let { id, expr } => {
                if variables.contains_key(&id) {
                    return Err(TypeCheckError::VariableAlreadyExists(id));
                }

                let typed_expr = type_check_expr(expr, &variables)?;
                variables.insert(id.clone(), typed_expr.get_type());

                Statement::Let {
                    id,
                    expr: typed_expr,
                }
            }
            ParsedStatement::DebugPrint(expr) => {
                let typed_expr = type_check_expr(expr, &variables)?;
                Statement::DebugPrint(typed_expr)
            }
        };
        typed_stmts.push(stmt);
    }

    Ok(Program {
        main_function: Function {
            name: main.name,
            statements: typed_stmts,
        },
    })
}

fn type_check_expr(
    expr: ParsedExpression,
    variables: &HashMap<Identifier, Type>,
) -> TypeCheckResult<Expression> {
    Ok(match expr {
        ParsedExpression::IntegerLiteral(i) => Expression::IntegerLiteral(i),
        ParsedExpression::Variable(identifier) => {
            let t = variables
                .get(&identifier)
                .ok_or_else(|| TypeCheckError::VariableNotFound(identifier.clone()))?
                .clone();

            Expression::Variable(identifier, t)
        }
    })
}
