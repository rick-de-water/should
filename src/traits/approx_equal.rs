use crate::expected::Expected;

pub trait ShouldBeApproximatelyEqual<T: ?Sized> {
    fn should_be_approx(&self, expected: impl Expected<T>, tolerance: impl Expected<T>);
    fn should_not_be_approx(&self, expected: impl Expected<T>, tolerance: impl Expected<T>);
}
