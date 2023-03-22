pub trait Expected<T> {
    fn value(&self) -> &T;
}

impl<T> Expected<T> for T {
    fn value(&self) -> &T {
        &self
    }
}

impl<T> Expected<T> for &T {
    fn value(&self) -> &T {
        self
    }
}