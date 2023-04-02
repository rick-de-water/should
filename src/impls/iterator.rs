use std::fmt::Debug;

use crate::{ShouldBeEmpty, caller_name::get_caller_name};

impl<T: Debug> ShouldBeEmpty for T where for<'a> &'a T: IntoIterator {
    fn should_be_empty(&self) {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self.into_iter().next() {
            Some(_) => panic!("{} should be empty but was {:?}", caller_name, self),
            None => ()
        }
    }

    fn should_not_be_empty(&self) {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self.into_iter().next() {
            Some(_) => (),
            None => panic!("{} should not be empty", caller_name)
        }
    }
}