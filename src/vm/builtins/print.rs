use super::PtyNull;
use crate::vm::{core::Vm, function_args::FuncArgs, object::PettyObject};

pub fn print(vm: &mut Vm, this: PettyObject, args: FuncArgs) -> PettyObject {
    for arg in args.0 {
        print!("{} ", arg.force_repr(vm));
    }
    println!();
    PtyNull.into()
}
