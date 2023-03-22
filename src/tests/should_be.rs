use crate::*;

#[test]
fn should_be_success() {
    let i1 = 21;
    let i2 = -3;
    let i3 = 21;
    let i4 = -3;

    21.should_be(21);
    (-3).should_be(-3);
    21.should_be(i1);
    (-3).should_be(i2);
    i1.should_be(21);
    i2.should_be(-3);

    i1.should_be(i3);
    i2.should_be(i4);
    i1.should_be(&i3);
    i2.should_be(&i4);
}

#[test]
#[should_panic(expected = "true should be false but was true")]
fn should_be_bool_fail_rvalue_true() {
    true.should_be(false);
}

#[test]
#[should_panic(expected = "false should be true but was false")]
fn should_be_bool_fail_rvalue_false() {
    false.should_be(true);
}

#[test]
#[should_panic(expected = "foo should be false but was true")]
fn should_be_bool_fail_lvalue_true() {
    let foo = true;
    foo.should_be(false);
}

#[test]
#[should_panic(expected = "bar should be true but was false")]
fn should_be_bool_fail_lvalue_false() {
    let bar = false;
    bar.should_be(true);

    let result: Result<i32, String> = Ok(21i32);

    result.should_be(21i32)
}