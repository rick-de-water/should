pub trait ShouldContain<T: ?Sized> {
    fn should_contain(&self, expected: impl Expected<T>);
}