use crate::{expected::Expected, ShouldBeEqual, ShouldBeOrdered, ShouldBeOption, caller_name::get_caller_name};
use std::fmt::Debug;

impl<T: PartialEq + Debug> ShouldBeEqual<T> for Option<T> {
    fn should_be(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value.should_be(expected.value()),
            None => panic!("{} should be {:?} but was None", caller_name, other)
        }
    }

    fn should_not_be(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_not_be(expected.value()),
            None => ()
        }
    }
}

impl<T: PartialOrd + Debug> ShouldBeOrdered<T> for Option<T> {
    fn should_be_lt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value.should_be_lt(expected.value()),
            None => panic!("{} should be less than {:?} but was {:?}", caller_name, other, self),
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value.should_be_le(expected.value()),
            None => panic!("{} should be less than or equal to {:?} but was {:?}", caller_name, other, self),
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value.should_be_gt(expected.value()),
            None => panic!("{} should be greater than {:?} but was {:?}", caller_name, other, self),
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value.should_be_ge(expected.value()),
            None => panic!("{} should be greater than or equal to {:?} but was {:?}", caller_name, other, self),
        }
    }
}

impl<T: Debug> ShouldBeOption<T> for Option<T> {
    fn should_be_some(&self) -> &T {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(value) => value,
            None => panic!("{} should be Some but was None", caller_name)
        }
    }

    fn should_be_none(&self) {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Some(_) => panic!("{} should be None but was {:?}", caller_name, self),
            None => ()
        }
    }
}