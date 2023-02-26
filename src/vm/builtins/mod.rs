mod null;
mod string;
mod int;

pub use null::PtyNull;
pub use string::PtyStr;
pub use int::PtyInt;

use crate::ast::Literal;

use super::{core::VirtualMachine, object::PettyObject};
pub fn load_builtins(vm: &mut VirtualMachine) {
}

pub fn create_literal(literal: &Literal) -> PettyObject {
    match literal  {
        Literal::Null => PtyNull.into(),
        Literal::Int(int) => PtyInt::new(*int).into(),
        _ => todo!()
    }
}