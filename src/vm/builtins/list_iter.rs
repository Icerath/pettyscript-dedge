use crate::vm::prelude::*;

#[derive(Clone)]
pub struct PtyListIter(pub Arc<Mutex<Vec<PettyObject>>>, pub Arc<Mutex<usize>>);

impl PettyObjectType for PtyListIter {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "__next__" => __NEXT__.clone(),
            "__iter__" | "iter" => __ITER__.clone(),
            "__repr__" => __REPR__.clone(),
            "__len__" | "len" => __LEN__.clone(),
            _ => todo!("{str}"),
        }
    }
    fn call(&self, _vm: &mut Vm, _this: &PettyObject, _args: FuncArgs) -> PettyObject {
        todo!()
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl fmt::Display for PtyListIter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "iterator at {:?}", self as *const Self)
    }
}

#[pettymethod]
fn __repr__(this: &PtyListIter) -> PtyStr {
    PtyStr(format!("{this}").into())
}

#[pettymethod]
fn __next__(this: &PtyListIter) -> PettyObject {
    let mut int = this.1.lock().unwrap();
    let next = this.0.lock().unwrap().get(*int).cloned();
    *int += 1;
    PtyOption::new(next)
}

#[pettymethod]
fn __iter__(this: PtyListIter) -> PtyListIter {
    this
}

#[pettymethod]
fn __len__(this: &PtyListIter) -> PtyNum {
    let total_len = this.0.lock().unwrap().len();
    let consumed = this.1.lock().unwrap();
    #[allow(clippy::cast_precision_loss)]
    PtyNum((total_len - *consumed) as f64)
}
