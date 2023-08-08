use core::ops::{Deref, DerefMut};

/// Convenient wrapper struct that implements any of the traits supported by
/// this crate if the contained type derefs to something implementing the
/// `**Obj` analog trait.
#[derive(Clone, Copy, Debug)]
pub struct Obj<T>(pub T);

impl<T> Obj<T> {
    /// Typically, you should just use Obj(item). This method is for
    /// compatibility with type aliases.
    pub fn new(item: T) -> Self {
        Obj(item)
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
