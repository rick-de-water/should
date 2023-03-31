pub trait Expected<T: ?Sized> {
    fn value(&self) -> &T;
}

impl<T: ?Sized> Expected<T> for T {
    fn value(&self) -> &T {
        &self
    }
}

impl<T: ?Sized> Expected<T> for &T {
    fn value(&self) -> &T {
        self
    }
}