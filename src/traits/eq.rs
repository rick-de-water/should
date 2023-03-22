use crate::expected::Expected;

pub trait ShouldEq<T> {
    fn should_be(&self, expected: impl Expected<T>);
    fn should_not_be(&self, expected: impl Expected<T>);
}
