mod builtins;
mod interpreter;
mod value;

use self::interpreter::Interpreter;
use crate::ast::Node;
use builtins::load_builtins;

pub fn interpret(ast: &Node) {
    let mut interpreter = Interpreter::init();
    load_builtins(&mut interpreter);
    interpreter.evaluate(ast);
}
