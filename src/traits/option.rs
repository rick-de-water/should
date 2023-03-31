pub trait ShouldOption<T: ?Sized> {
    fn should_be_some(&self);
    fn should_be_none(&self);
}
