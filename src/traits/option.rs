pub trait ShouldOption<T> {
    fn should_be_some(&self);
    fn should_be_none(&self);
}
