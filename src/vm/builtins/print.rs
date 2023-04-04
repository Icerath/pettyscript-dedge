use super::PtyNull;
use crate::vm::{core::Vm, function_args::FuncArgs, object::PettyObject};

pub fn print(vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
    for arg in args.0 {
        if let Some(repr) = arg.repr(vm) {
            print!("{} ", repr.0);
        } else {
            eprintln!("\nTODO - {arg}"); // TODO
        }
    }
    PtyNull.into()
}
