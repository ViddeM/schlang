use std::collections::HashMap;

use types::Value;

use crate::{
    common_types::Identifier,
    type_checking::types::{Expression, Program, Statement},
};

pub mod types;

#[derive(thiserror::Error, Debug)]
pub enum InterpretError {
    #[error(
        "Variable `{0}` not found (unexpected, this should've been checked during type-check!"
    )]
    VariableNotFound(Identifier),
}

type InterpretResult<T> = Result<T, InterpretError>;

pub fn interpret(program: Program) -> InterpretResult<()> {
    let mut variables: HashMap<Identifier, Value> = HashMap::new();

    for stmt in program.main_function.statements {
        match stmt {
            Statement::Let { id, expr } => {
                let expr_value = evaluate_expr(expr, &variables)?;
                variables.insert(id, expr_value);
            }
            Statement::DebugPrint(expr) => {
                let expr_value = evaluate_expr(expr, &variables)?;
                debug_print(expr_value);
            }
        }
    }

    Ok(())
}

fn evaluate_expr(
    expr: Expression,
    variables: &HashMap<Identifier, Value>,
) -> InterpretResult<Value> {
    Ok(match expr {
        Expression::IntegerLiteral(i) => Value::Integer(i),
        Expression::Variable(ident, _) => variables
            .get(&ident)
            .ok_or(InterpretError::VariableNotFound(ident))?
            .clone(),
    })
}

fn debug_print(value: Value) {
    println!("DBG VALUE {value:?}");
}
