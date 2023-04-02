use crate::{expected::Expected, ShouldBeEqual, ShouldBeOrdered, ShouldBeResult, caller_name::get_caller_name};
use std::fmt::Debug;

impl<T: PartialEq + Debug, E: Debug> ShouldBeEqual<T> for Result<T, E> {
    fn should_be(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value.should_be(expected.value()),
            Err(_) => panic!("{} should be {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_not_be(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_not_be(expected.value()),
            Err(_) => (),
        }
    }
}

impl<T: PartialOrd + Debug, E: Debug> ShouldBeOrdered<T> for Result<T, E> {
    fn should_be_lt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value.should_be_lt(expected.value()),
            Err(_) => panic!("{} should be less than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value.should_be_le(expected.value()),
            Err(_) => panic!("{} should be less than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value.should_be_gt(expected.value()),
            Err(_) => panic!("{} should be greater than {:?} but was {:?}", caller_name, other, self)
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        let other = expected.value();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value.should_be_ge(expected.value()),
            Err(_) => panic!("{} should be greater than or equal to {:?} but was {:?}", caller_name, other, self)
        }
    }
}

impl<T: Debug, E: Debug> ShouldBeResult<T, E> for Result<T, E> {
    fn should_be_ok(&self) -> &T {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(value) => value,
            Err(_) => panic!("{} should be Ok but was {:?}", caller_name, self)
        }
    }

    fn should_be_err(&self) -> &E {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(_) => panic!("{} should be Err but was {:?}", caller_name, self),
            Err(value) => value
        }
    }
}