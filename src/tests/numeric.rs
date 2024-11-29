use crate::ShouldBeApproximatelyEqual;


#[test]
fn success() {
    let value =  0.25;

    value.should_be_approx(0.0, 0.25);
    value.should_be_approx(0.25, 0.25);
    value.should_be_approx(0.5, 0.25);
    value.should_not_be_approx(0.75, 0.25);
}


#[test]
#[should_panic(expected = "0.25 should be between 0.5 and 1.0 but was 0.25")]
fn should_be_approx_fail() {
    0.25.should_be_approx(0.75, 0.25);
}

#[test]
#[should_panic(expected = "0.25 should not be between -0.25 and 0.25 but was 0.25")]
fn should_not_be_approx_fail() {
    0.25.should_not_be_approx(0.0, 0.25);
}
