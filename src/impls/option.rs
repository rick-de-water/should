use crate::{expected::Expected, ShouldEq, ShouldOrd, ShouldOption};
use std::fmt::Debug;

impl<T: PartialEq + Debug> ShouldEq<T> for Option<T> {
    fn should_be(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be(expected.value()),
            None => panic!(),
        }
    }

    fn should_not_be(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be(expected.value()),
            None => panic!(),
        }
    }
}

impl<T: PartialOrd + Debug> ShouldOrd<T> for Option<T> {
    fn should_be_lt(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be_lt(expected.value()),
            None => panic!(),
        }
    }

    fn should_be_le(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be_le(expected.value()),
            None => panic!(),
        }
    }

    fn should_be_gt(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be_gt(expected.value()),
            None => panic!(),
        }
    }

    fn should_be_ge(&self, expected: impl Expected<T>) {
        match self {
            Some(value) => value.should_be_ge(expected.value()),
            None => panic!(),
        }
    }
}

impl<T: Debug> ShouldOption<T> for Option<T> {
    fn should_be_some(&self) {
        match self {
            Some(_) => (),
            None => panic!()
        }
    }

    fn should_be_none(&self) {
        match self {
            Some(_) => panic!(),
            None => ()
        }
    }
}