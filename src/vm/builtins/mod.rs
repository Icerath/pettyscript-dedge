mod bool;
mod function_template;
mod int;
mod null;
mod string;

pub use int::PtyInt;
pub use null::PtyNull;
pub use string::PtyStr;

use crate::ast::Literal;

use super::{
    core::{VirtualMachine},
    object::{PettyObject},
};
pub fn load_builtins(vm: &mut VirtualMachine) {}

pub fn create_literal(literal: &Literal) -> PettyObject {
    match literal {
        Literal::Null => PtyNull.into(),
        Literal::Int(int) => PtyInt::new(*int).into(),
        _ => todo!(),
    }
}
