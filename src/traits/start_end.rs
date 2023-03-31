pub trait ShouldStartEndString {
    fn should_start_with(&self, expected: impl Into<String>);
    fn should_not_start_with(&self, expected: impl Into<String>);
    fn should_end_with(&self, expected: impl Into<String>);
    fn should_not_end_with(&self, expected: impl Into<String>);
}