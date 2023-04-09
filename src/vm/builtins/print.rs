use std::io::{self, Write};

use macros::pettymethod;

use crate::vm::{core::Vm, function_args::FuncArgs};

#[pettymethod]
pub fn print(vm: &mut Vm, args: FuncArgs) {
    let reprs: Vec<_> = args.0.iter().map(|arg| arg.force_repr(vm).0).collect();
    let stdout = io::stdout();

    let mut guard = stdout.lock();
    for repr in reprs {
        let _ = write!(guard, "{repr}");
    }
    let _ = writeln!(guard);
}
