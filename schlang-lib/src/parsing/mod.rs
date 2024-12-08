use lalrpop_util::lalrpop_mod;
use types::Program;

pub mod grammar;
pub mod types;

lalrpop_mod!(core, "/parsing/grammar/core.rs");

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Failed to parse source file err: `{0}`")]
    LalrpopError(String),
}

pub fn parse(program: &str) -> Result<Program, ParseError> {
    let program = core::PrgrParser::new()
        .parse(program)
        .or_else(|err| Err(ParseError::LalrpopError(format!("{err}"))))?;

    Ok(program)
}
