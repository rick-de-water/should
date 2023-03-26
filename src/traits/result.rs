pub trait ShouldResult<T> {
    fn should_be_ok(&self);
    fn should_be_err(&self);
}