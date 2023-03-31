use crate::expected::Expected;

pub trait ShouldOrd<T: ?Sized> {
    fn should_be_lt(&self, expected: impl Expected<T>);
    fn should_be_le(&self, expected: impl Expected<T>);
    fn should_be_gt(&self, expected: impl Expected<T>);
    fn should_be_ge(&self, expected: impl Expected<T>);
}
