use super::{
    interpreter::Interpreter,
    value::{PettyObject, PettyValue},
};
pub fn load_builtins(interpreter: &mut Interpreter) {
    let builtins = [("print", Box::new(PrintBuiltin))];
    for (name, builtin) in builtins {
        interpreter.load_builtin((*name).into(), PettyValue::new(builtin));
    }
}
// Builtins
pub struct IntBuiltin(pub i128);
pub struct FloatBuiltin(pub f64);
pub struct BoolBuiltin(pub bool);
pub struct StringBuiltin(pub String);
pub struct NullBuiltin;
struct PrintBuiltin;

// Builtin Implementations
impl PettyObject for StringBuiltin {
    fn type_name(&self) -> &'static str {
        "String"
    }
    fn __add__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let other = &other.inner().as_any().downcast_ref::<StringBuiltin>()?.0;
        Some(Self(self.0.clone() + other).into())
    }
    fn __repr__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        Some(source)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl PettyObject for NullBuiltin {
    fn type_name(&self) -> &'static str {
        "null"
    }
    fn __bool__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        Some(BoolBuiltin(false).into())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl PettyObject for BoolBuiltin {
    fn type_name(&self) -> &'static str {
        "bool"
    }
    fn __bool__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        Some(source)
    }
    fn __and__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let other = other.inner().as_any().downcast_ref::<Self>()?;
        Some(Self(self.0 && other.0).into())
    }
    fn __or__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let other = other.inner().as_any().downcast_ref::<Self>()?;
        Some(Self(self.0 && other.0).into())
    }
    fn __repr__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        Some(StringBuiltin(format!("{}", self.0)).into())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PettyObject for IntBuiltin {
    fn type_name(&self) -> &'static str {
        "int"
    }
    fn __add__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        Some(match other.inner().as_any() {
            int if int.downcast_ref::<Self>().is_some() => {
                let other = int.downcast_ref::<Self>().unwrap();
                Self(self.0 + other.0).into()
            }
            float if float.downcast_ref::<FloatBuiltin>().is_some() => {
                let other = float.downcast_ref::<FloatBuiltin>().unwrap();
                FloatBuiltin(self.0 as f64 + other.0).into()
            }
            bool if bool.downcast_ref::<BoolBuiltin>().is_some() => {
                let other = bool.downcast_ref::<BoolBuiltin>().unwrap();
                IntBuiltin(self.0 + i128::from(other.0)).into()
            }
            _ => return None,
        })
    }
    fn __sub__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let inner = other.inner().as_any().downcast_ref::<Self>()?;
        Some(Self(self.0 - inner.0).into())
    }
    fn __mul__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let inner = other.inner().as_any().downcast_ref::<Self>()?;
        Some(Self(self.0 - inner.0).into())
    }
    fn __div__(
        &self,
        interpreter: &mut Interpreter,
        source: PettyValue,
        other: PettyValue,
    ) -> Option<PettyValue> {
        let inner = other.inner().as_any().downcast_ref::<Self>()?;
        Some(Self(self.0 - inner.0).into())
    }
    fn __lt__(&self, other: PettyValue) -> Option<PettyValue> {
        Some(match other.inner().as_any() {
            int if int.downcast_ref::<Self>().is_some() => {
                let other = int.downcast_ref::<Self>().unwrap();
                BoolBuiltin(self.0 < other.0).into()
            }
            float if float.downcast_ref::<FloatBuiltin>().is_some() => {
                let other = float.downcast_ref::<FloatBuiltin>().unwrap();
                BoolBuiltin((self.0 as f64) < other.0).into()
            }
            _ => return None,
        })
    }
    fn __gt__(&self, other: PettyValue) -> Option<PettyValue> {
        Some(match other.inner().as_any() {
            int if int.downcast_ref::<Self>().is_some() => {
                let other = int.downcast_ref::<Self>().unwrap();
                BoolBuiltin(self.0 > other.0).into()
            }
            float if float.downcast_ref::<FloatBuiltin>().is_some() => {
                let other = float.downcast_ref::<FloatBuiltin>().unwrap();
                BoolBuiltin((self.0 as f64) > other.0).into()
            }
            _ => return None,
        })
    }
    fn __lt_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        Some(match other.inner().as_any() {
            int if int.downcast_ref::<Self>().is_some() => {
                let other = int.downcast_ref::<Self>().unwrap();
                BoolBuiltin(self.0 <= other.0).into()
            }
            float if float.downcast_ref::<FloatBuiltin>().is_some() => {
                let other = float.downcast_ref::<FloatBuiltin>().unwrap();
                BoolBuiltin((self.0 as f64) <= other.0).into()
            }
            _ => return None,
        })
    }
    fn __gt_eq__(&self, other: PettyValue) -> Option<PettyValue> {
        Some(match other.inner().as_any() {
            int if int.downcast_ref::<Self>().is_some() => {
                let other = int.downcast_ref::<Self>().unwrap();
                BoolBuiltin(self.0 >= other.0).into()
            }
            float if float.downcast_ref::<FloatBuiltin>().is_some() => {
                let other = float.downcast_ref::<FloatBuiltin>().unwrap();
                BoolBuiltin((self.0 as f64) >= other.0).into()
            }
            _ => return None,
        })
    }
    fn __repr__(&self, interpreter: &mut Interpreter, source: PettyValue) -> Option<PettyValue> {
        Some(StringBuiltin(self.0.to_string()).into())
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PettyObject for FloatBuiltin {
    fn type_name(&self) -> &'static str {
        "float"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PettyObject for PrintBuiltin {
    fn type_name(&self) -> &'static str {
        "function"
    }
    fn __call__(&self, interpreter: &mut Interpreter, args: Vec<PettyValue>) -> Option<PettyValue> {
        for arg in args {
            let repr = arg.inner().__repr__(interpreter, arg.clone()).unwrap();
            let string = repr
                .inner()
                .as_any()
                .downcast_ref::<StringBuiltin>()
                .unwrap();
            println!("{}", string.0);
        }
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
