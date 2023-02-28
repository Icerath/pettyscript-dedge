use std::{fmt, ops::Deref};

use crate::slim_rc::Rc;

#[derive(PartialEq, Clone)]
pub struct RcStr(Rc<str>);
impl From<&str> for RcStr {
    fn from(value: &str) -> Self {
        Self(Rc::from(value))
        // Self(Rc {
        //     ref_count: Box::leak(Box::new(1)).into(),
        //     object: String::from(value).as_str().into(),
        // })
    }
}
impl From<String> for RcStr {
    fn from(value: String) -> Self {
        Self(Rc::from(value))
    }
}
impl fmt::Display for RcStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Debug for RcStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Deref for RcStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
