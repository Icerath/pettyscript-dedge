use macros::pettymethod;

use crate::vm::builtins::PtyNum;

#[pettymethod]
pub fn sleep(duration: PtyNum) {
    std::thread::sleep(std::time::Duration::from_secs_f64(duration.0));
}
