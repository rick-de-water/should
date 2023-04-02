use crate::{ShouldBeEqual, ShouldBeOrdered, ShouldBeResult};

#[test]
fn success() {
    let result =  Ok::<i32, ()>(14);

    result.should_be(14);
    result.should_not_be(12);
    result.should_be_lt(15);
    result.should_be_le(14);
    result.should_be_gt(13);
    result.should_be_ge(14);
    result.should_be_ok();
    Err(()).should_not_be(12);
}

#[test]
#[should_panic(expected = "Err::<i32, ()>(()) should be Ok but was Err(())")]
fn should_be_ok_fail() {
    Err::<i32, ()>(()).should_be_ok();
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be Err but was Ok(14)")]
fn should_be_err_fail() {
    Ok::<i32, ()>(14).should_be_err();
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be 12 but was 14")]
fn should_be_fail_ok() {
    Ok::<i32, ()>(14).should_be(12);
}

#[test]
#[should_panic(expected = "Err(()) should be 12 but was Err(())")]
fn should_be_fail_err() {
    Err(()).should_be(12);
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should not be 14")]
fn should_not_be_fail() {
    Ok::<i32, ()>(14).should_not_be(14);
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be greater than 15 but was 14")]
fn should_be_gt_fail_ok() {
    Ok::<i32, ()>(14).should_be_gt(15);
}

#[test]
#[should_panic(expected = "Err(()) should be greater than 15 but was Err(())")]
fn should_be_gt_fail_err() {
    Err(()).should_be_gt(15);
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be greater than or equal to 15 but was 14")]
fn should_be_ge_fail_ok() {
    Ok::<i32, ()>(14).should_be_ge(15);
}

#[test]
#[should_panic(expected = "Err(()) should be greater than or equal to 15 but was Err(())")]
fn should_be_ge_fail_err() {
    Err(()).should_be_ge(15);
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be less than 13 but was 14")]
fn should_be_lt_fail_ok() {
    Ok::<i32, ()>(14).should_be_lt(13);
}

#[test]
#[should_panic(expected = "Err(()) should be less than 13 but was Err(())")]
fn should_be_lt_fail_err() {
    Err(()).should_be_lt(13);
}

#[test]
#[should_panic(expected = "Ok::<i32, ()>(14) should be less than or equal to 13 but was 14")]
fn should_be_le_fail_ok() {
    Ok::<i32, ()>(14).should_be_le(13);
}

#[test]
#[should_panic(expected = "Err(()) should be less than or equal to 13 but was Err(())")]
fn should_be_le_fail_err() {
    Err(()).should_be_le(13);
}