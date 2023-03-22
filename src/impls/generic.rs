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
        if self < other {
            panic!("{} should be less than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self <= other {
            panic!("{} should be less than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self > other {
            panic!("{} should be greater than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if self >= other {
            panic!("{} should be greater than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }
}
