pub trait ShouldBeOption<T: ?Sized> {
    fn should_be_some(&self);
    fn should_be_none(&self);
}
