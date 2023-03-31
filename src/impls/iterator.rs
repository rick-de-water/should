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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succeed() {
        let v1 = Vec::<i32>::new();
        let v2 = vec![1, 2, 3];

        v1.should_be_empty();
        v2.should_not_be_empty();

        Vec::<i32>::new().should_be_empty();
        vec![1, 2, 3].should_not_be_empty();
    }

    #[test]
    #[should_panic(expected = "vec![1, 2, 3] should be empty but was [1, 2, 3]")]
    fn should_be_empty_fail() {
        vec![1, 2, 3].should_be_empty();
    }

    #[test]
    #[should_panic(expected = "Vec::<i32>::new() should not be empty")]
    fn should_not_be_empty_fail() {
        Vec::<i32>::new().should_not_be_empty();
    }
}