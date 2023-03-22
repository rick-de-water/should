use crate::*;

#[test]
fn should_not_be_success() {
    let i1 = 21;
    let i2 = -3;

    21.should_not_be(-3);
    (-3).should_not_be(21);
    21.should_not_be(i2);
    (-3).should_not_be(i1);
    i1.should_not_be(-3);
    i2.should_not_be(21);

    i1.should_not_be(i2);
    i2.should_not_be(i1);
    i1.should_not_be(&i2);
    i2.should_not_be(&i1);
}