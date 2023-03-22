use crate::{expected::Expected, ShouldEq, ShouldOrd, ShouldResult};
use std::fmt::Debug;

impl<T: PartialEq + Debug, E: Debug> ShouldEq<T> for Result<T, E> {
    fn should_be(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_be(expected.value()),
            Err(_) => panic!(),
        }
    }

    fn should_not_be(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_not_be(expected.value()),
            Err(_) => panic!(),
        }
    }
}

impl<T: PartialOrd + Debug, E: Debug> ShouldOrd<T> for Result<T, E> {
    fn should_be_lt(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_be_lt(expected.value()),
            Err(_) => panic!(),
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_be_le(expected.value()),
            Err(_) => panic!(),
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_be_gt(expected.value()),
            Err(_) => panic!(),
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        match self {
            Ok(value) => value.should_be_ge(expected.value()),
            Err(_) => panic!(),
        }
    }
}

impl<T: Debug, E: Debug> ShouldResult<T> for Result<T, E> {
    fn should_be_ok(&self) {
        match self {
            Ok(_) => (),
            Err(_) => panic!()
        }
    }

    fn should_be_err(&self) {
        match self {
            Ok(_) => panic!(),
            Err(_) => ()
        }
    }
}