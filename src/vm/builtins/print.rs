use macros::pettymethod;

use crate::vm::{core::Vm, function_args::FuncArgs};

#[pettymethod]
pub fn print(vm: &mut Vm, args: FuncArgs) {
    for arg in args.0 {
        print!("{} ", arg.force_repr(vm));
    }
    println!();
}
