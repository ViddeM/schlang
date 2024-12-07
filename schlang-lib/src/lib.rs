mod common_types;
mod interpreter;
mod parsing;
mod type_checking;

pub fn run(program: &str) {
    let parsed_program = parsing::parse(program).expect("Failed to parse application");
    let typed_program = type_checking::type_check(parsed_program).expect("Typecheck failed");
    interpreter::interpret(typed_program).expect("Failed to interpret program");
}
