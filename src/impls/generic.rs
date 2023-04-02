use crate::{expected::Expected, caller_name::get_caller_name, ShouldBeEqual, ShouldBeOrdered, ShouldBeSame};
use std::fmt::Debug;

impl<T: PartialEq + Debug> ShouldBeEqual<T> for T {
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

impl<T: PartialOrd + Debug> ShouldBeOrdered<T> for T {
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

impl<T> ShouldBeSame for T {
    fn should_be_same(&self, other: &Self) {
        if self as *const T != other as *const T {
            let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
            panic!("{caller_name} should be the same as {:p} but was {:p}", other, self)
        }
    }

    fn should_not_be_same(&self, other: &Self) {
        if self as *const T == other as *const T {
            let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
            panic!("{caller_name} should not be the same as {:p}", other)
        }
    }
}