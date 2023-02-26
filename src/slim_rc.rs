use std::{fmt, ops::Deref, ptr::NonNull};

#[repr(C)]
pub struct Rc<T: ?Sized> {
    pub ref_count: NonNull<usize>,
    pub object: NonNull<T>,
}
impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        Self {
            ref_count: Box::leak(Box::new(1)).into(),
            object: Box::leak(Box::new(value)).into(),
        }
    }
}

impl<T: ?Sized> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            *self.ref_count.as_ptr() += 1;
        }
        Self {
            ref_count: self.ref_count,
            object: self.object,
        }
    }
}

impl<T: ?Sized> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            *self.ref_count.as_ptr() -= 1;
            if *self.ref_count.as_ptr() == 0 {
                self.object.as_ptr().drop_in_place()
            }
        }
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{}", self.object.as_ref()) }
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for Rc<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { write!(f, "{:?}", self.object.as_ref()) }
    }
}

impl<T: ?Sized> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.object.as_ref() }
    }
}
