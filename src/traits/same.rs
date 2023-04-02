pub trait ShouldBeSame {
    fn should_be_same(&self, other: &Self);
    fn should_not_be_same(&self, other: &Self);
}