use std::{marker::PhantomData, ops::Deref};


#[derive(Debug)]
pub struct PtrWraper<'a,T> (T, PhantomData<&'a ()>);

impl<'a, T> PtrWraper<'a, T> {
    pub fn new(ptr: T) -> Self {
        PtrWraper(ptr, PhantomData)
    }
}

impl<T> Deref for PtrWraper<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
