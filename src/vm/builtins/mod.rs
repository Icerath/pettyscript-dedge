pub mod function_template;

mod int;
mod null;
mod print;
mod pty_bool;
mod repr;
mod string;

pub use self::repr::PtyRepr;
pub use function_template::{display_class_object, display_function_object};
pub use int::PtyInt;
pub use null::PtyNull;
pub use print::PtyPrint;
pub use pty_bool::PtyBool;
pub use string::PtyStr;

use crate::ast::Literal;

use super::{core::VirtualMachine, object::PettyObject};
pub fn load_builtins(vm: &mut VirtualMachine) {
    let builtins = [("print", PtyPrint.into()), ("repr", PtyRepr.into())];
    for (name, builtin) in builtins {
        vm.load_builtin(name, builtin);
    }
}

pub fn create_literal(literal: &Literal) -> PettyObject {
    match literal {
        Literal::Int(int) => PtyInt(*int).into(),
        Literal::Null => PtyNull.into(),
        Literal::Bool(bool) => PtyBool(*bool).into(),
        Literal::String(string) => PtyStr(string.clone()).into(),
        _ => todo!(),
    }
}
