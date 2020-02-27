pub mod function;
pub mod interpreter;
pub mod rpn;
pub mod token;
pub mod tree;

fn main() {
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.begin().unwrap();
}
