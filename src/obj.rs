use core::ops::{Deref, DerefMut};
use std::{rc::Rc, sync::Arc};

/// Convenient wrapper struct that implements any of the traits supported by
/// this crate if the contained type derefs to something implementing the
/// `**Obj` analog trait.
#[derive(Clone, Copy, Debug)]
pub struct Obj<T>(pub T);

impl<T> Obj<Box<T>> {
    pub fn boxed(item: T) -> Self {
        Obj(Box::new(item))
    }
}

impl<T> Obj<Rc<T>> {
    pub fn rc(item: T) -> Self {
        Obj(Rc::new(item))
    }
}

impl<T> Obj<Arc<T>> {
    pub fn arc(item: T) -> Self {
        Obj(Arc::new(item))
    }
}

impl<T> Deref for Obj<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Obj<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
