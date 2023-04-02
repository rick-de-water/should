pub trait ShouldBeResult<T: ?Sized, E: ?Sized> {
    fn should_be_ok(&self) -> &T;
    fn should_be_err(&self) -> &E;
}