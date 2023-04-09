use crate::vm::prelude::*;

#[derive(Clone)]
pub struct PtyRange {
    end: f64,
    step: f64,
    current: Arc<Mutex<f64>>,
}

impl PtyRange {
    #[inline]
    pub fn next(&self) -> Option<f64> {
        let mut current = self.current.lock().unwrap();
        let prev_current = *current;
        *current += self.step;
        if *current > self.end {
            return None;
        }
        Some(prev_current)
    }
}

impl PettyObjectType for PtyRange {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "__iter__" | "iter" => __ITER__.clone(),
            "__next__" | "next" => __NEXT__.clone(),
            "__len__" | "len" => __LEN__.clone(),
            "__repr__" => __REPR__.clone(),
            "sum" => SUM.clone(),

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

impl fmt::Display for PtyRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "range at {:?}", self as *const Self)
    }
}

#[pettymethod]
pub fn range(end: PtyNum) -> PtyRange {
    PtyRange {
        end: end.0,
        current: Mutex::new(0.0).into(),
        step: 1.0,
    }
}

#[pettymethod]
fn __iter__(this: PtyRange) -> PtyRange {
    this
}

#[pettymethod]
fn __next__(this: &PtyRange) -> PettyObject {
    match this.next() {
        Some(num) => PtyOption(Some(PtyNum(num).into())).into(),
        None => NONE.clone(),
    }
}

#[pettymethod]
fn __len__(_this: PtyRange) -> PtyNum {
    todo!()
}

#[pettymethod]
fn __repr__(this: PtyRange) -> PtyStr {
    PtyStr(format!("{this}").into())
}

#[pettymethod]
fn sum(this: &PtyRange) -> PtyNum {
    let mut sum = 0.0;
    while let Some(next) = this.next() {
        sum += next;
    }
    PtyNum(sum)
}
