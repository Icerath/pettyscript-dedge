use crate::vm::prelude::*;

#[derive(Clone)]
pub struct PtyList(pub Arc<Mutex<Vec<PettyObject>>>);

impl PettyObjectType for PtyList {
    fn get_item(&self, _vm: &mut Vm, _this: &PettyObject, str: &str) -> PettyObject {
        match str {
            "len" => LEN.clone(),
            "push" => PUSH.clone(),
            "get" | "__get_index__" => GET.clone(),
            "set" | "__set_index__" => SET.clone(),
            "contains" => CONTAINS.clone(),
            "find" => FIND.clone(),
            "__repr__" => __REPR__.clone(),
            "__add__" => __ADD__.clone(),
            "__mul__" => __MUL__.clone(),
            "__bool__" => __BOOL__.clone(),
            "__iter__" | "iter" => __ITER__.clone(),
            "__is_eq__" => __IS_EQ__.clone(),
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

impl fmt::Display for PtyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (index, item) in self.0.lock().unwrap().iter().enumerate() {
            let seperator = if index == 0 { "" } else { ", " };
            write!(f, "{seperator}{item}")?;
        }

        write!(f, "]")
    }
}

#[pettymethod]
#[allow(clippy::cast_precision_loss)]
fn len(this: &PtyList) -> PtyNum {
    PtyNum(this.0.lock().unwrap().len() as f64)
}

#[pettymethod]
fn get(this: &PtyList, index: PtyNum) -> PettyObject {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let index = index.0.max(0.0) as usize;
    PtyOption::new(this.0.lock().unwrap().get(index).cloned())
}

#[pettymethod]
fn set(this: &PtyList, index: PtyNum, obj: &PettyObject) {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let index = index.0.max(0.0) as usize;
    this.0.lock().unwrap()[index] = obj.clone();
}

#[pettymethod]
fn push(this: &PtyList, obj: &PettyObject) {
    this.0.lock().unwrap().push(obj.clone());
}

#[pettymethod]
fn find(this: &PtyList, obj: &PettyObject, vm: &mut Vm) -> PettyObject {
    let __is_eq__ = obj.get_item(vm, obj, "__is_eq__");
    // Not sure about this lock.
    let lock = this.0.lock().unwrap();
    for (index, object) in lock.iter().enumerate() {
        let bool = __is_eq__.call(vm, &__is_eq__, FuncArgs(&[obj, object]));
        let bool = bool.downcast::<PtyBool>().expect("Expected Bool");
        if bool.0 {
            return PtyOption(Some(PtyNum(index as f64).into())).into();
        }
    }
    NONE.clone()
}

#[pettymethod]
fn contains(this: &PtyList, obj: &PettyObject, vm: &mut Vm) -> PettyObject {
    let __is_eq__ = obj.get_item(vm, obj, "__is_eq__");
    // Not sure about this lock.
    let lock = this.0.lock().unwrap();
    for object in lock.iter() {
        let bool = __is_eq__.call(vm, &__is_eq__, FuncArgs(&[obj, object]));
        let bool = bool.downcast::<PtyBool>().expect("Expected Bool");
        if bool.0 {
            return TRUE.clone();
        }
    }
    FALSE.clone()
}

#[pettymethod]
fn __repr__(this: &PtyList, vm: &mut Vm) -> PtyStr {
    let mut string = String::from("[");
    for (index, item) in this.0.lock().unwrap().iter().enumerate() {
        let seperator = if index == 0 { "" } else { ", " };
        string.push_str(seperator);
        string.push_str(&item.force_repr(vm).0);
    }
    string.push(']');
    PtyStr(string.into())
}

#[pettymethod]
fn __bool__(this: &PtyList) -> PettyObject {
    PtyBool::new(!this.0.lock().unwrap().is_empty())
}

#[pettymethod]
fn __add__(lhs: &PtyList, rhs: &PtyList) -> PtyList {
    let mut vec = { lhs.0.lock().unwrap().clone() };
    vec.extend_from_slice(&rhs.0.lock().unwrap());

    PtyList(Mutex::new(vec).into())
}

#[pettymethod]
fn __mul__(lhs: &PtyList, rhs: PtyNum) -> PtyList {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    let repeat = rhs.0.max(0.0) as usize;
    let mut vec = Vec::with_capacity(repeat * lhs.0.lock().unwrap().len());
    for _ in 0..repeat {
        for obj in lhs.0.lock().unwrap().iter() {
            vec.push(obj.clone());
        }
    }
    PtyList(Mutex::new(vec).into())
}

#[pettymethod]
fn __is_eq__(lhs: &PtyList, rhs: &PtyList, vm: &mut Vm) -> PettyObject {
    let lhs_len = lhs.0.lock().unwrap().len();
    let rhs_len = rhs.0.lock().unwrap().len();
    if lhs_len != rhs_len {
        return FALSE.clone();
    }
    for index in 0..lhs_len {
        let lhs = &lhs.0.lock().unwrap()[index];
        let rhs = &rhs.0.lock().unwrap()[index];
        let is_eq = lhs.call_method(vm, "__is_eq__", FuncArgs(&[lhs, rhs]));
        let is_eq = is_eq.downcast::<PtyBool>().expect("Expected Bool");
        if !is_eq.0 {
            return FALSE.clone();
        }
    }
    TRUE.clone()
}

#[pettymethod]
fn __iter__(this: PtyList) -> PtyListIter {
    PtyListIter(this.0, Mutex::new(0).into())
}
