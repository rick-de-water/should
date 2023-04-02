use crate::{ShouldBeEqual, ShouldBeOrdered, ShouldBeOption};

#[test]
fn success() {
    let option =  Some(14);

    option.should_be(14);
    option.should_not_be(12);
    option.should_be_lt(15);
    option.should_be_le(14);
    option.should_be_gt(13);
    option.should_be_ge(14);
    option.should_be_some();

    None.should_not_be(12);
}

#[test]
#[should_panic(expected = "None::<i32> should be Some but was None")]
fn should_be_some_fail() {
    None::<i32>.should_be_some().should_be(14);
}

#[test]
#[should_panic(expected = "Some(14) should be None but was Some(14)")]
fn should_be_none_fail() {
    Some(14).should_be_none();
}

#[test]
#[should_panic(expected = "Some(14) should be 12 but was 14")]
fn should_be_fail_some() {
    Some(14).should_be(12)
}
#[test]
#[should_panic(expected = "None should be 12 but was None")]
fn should_be_fail_none() {
    None.should_be(12);
}

#[test]
#[should_panic(expected = "Some(14) should not be 14")]
fn should_not_be_fail() {
    Some(14).should_not_be(14);
}

#[test]
#[should_panic(expected = "Some(14) should be less than 13 but was 14")]
fn should_be_lt_fail_some() {
    Some(14).should_be_lt(13);
}

#[test]
#[should_panic(expected = "None should be less than 13 but was None")]
fn should_be_lt_fail_none() {
    None.should_be_lt(13);
}

#[test]
#[should_panic(expected = "Some(14) should be less than or equal to 13 but was 14")]
fn should_be_le_fail_some() {
    Some(14).should_be_le(13);
}

#[test]
#[should_panic(expected = "None should be less than or equal to 13 but was None")]
fn should_be_le_fail_none() {
    None.should_be_le(13);
}

#[test]
#[should_panic(expected = "Some(14) should be greater than 15 but was 14")]
fn should_be_gt_fail_some() {
    Some(14).should_be_gt(15);
}

#[test]
#[should_panic(expected = "None should be greater than 15 but was None")]
fn should_be_gt_fail_none() {
    None.should_be_gt(15);
}

#[test]
#[should_panic(expected = "Some(14) should be greater than or equal to 15 but was 14")]
fn should_be_ge_fail_some() {
    Some(14).should_be_ge(15);
}

#[test]
#[should_panic(expected = "None should be greater than or equal to 15 but was None")]
fn should_be_ge_fail_none() {
    None.should_be_ge(15);
}