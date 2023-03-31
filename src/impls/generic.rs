use crate::{expected::Expected, caller_name::get_caller_name, ShouldEq, ShouldOrd};
use std::fmt::Debug;

impl<T: PartialEq + Debug> ShouldEq<T> for T {
    fn should_be(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self != other {
            panic!("{} should be {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_not_be(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self == other {
            panic!("{} should not be {:?}", caller_name, other)
        }
    }
}

impl<T: PartialOrd + Debug> ShouldOrd<T> for T {
    fn should_be_lt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self >= other {
            panic!("{} should be less than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self > other {
            panic!("{} should be less than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self <= other {
            panic!("{} should be greater than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self < other {
            panic!("{} should be greater than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
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

        2.should_be_gt(1);
        2.should_be_ge(1);
        2.should_be_ge(2);
        1.should_be_lt(2);
        1.should_be_le(2);
        1.should_be_le(1);

    }

    #[test]
    #[should_panic(expected = "14 should be 12 but was 14")]
    fn should_be_fail() {
        14.should_be(12)
    }

    #[test]
    #[should_panic(expected = "14 should not be 14")]
    fn should_not_be_fail() {
        14.should_not_be(14);
    }

    #[test]
    #[should_panic(expected = "14 should be greater than 15 but was 14")]
    fn should_be_gt_fail() {
        14.should_be_gt(15)
    }

    #[test]
    #[should_panic(expected = "14 should be greater than or equal to 15 but was 14")]
    fn should_be_ge_fail() {
        14.should_be_ge(15)
    }

    #[test]
    #[should_panic(expected = "14 should be less than 13 but was 14")]
    fn should_be_lt_fail() {
        14.should_be_lt(13)
    }

    #[test]
    #[should_panic(expected = "14 should be less than or equal to 13 but was 14")]
    fn should_be_le_fail() {
        14.should_be_le(13)
    }
}