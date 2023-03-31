use crate::{expected::Expected, ShouldEq, ShouldOrd, ShouldResult, caller_name::get_caller_name};
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
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(_) => (),
            Err(_) => panic!("{} should be Ok but was {:?}", caller_name, self)
        }
    }

    fn should_be_err(&self) {
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        match self {
            Ok(_) => panic!("{} should be Err but was {:?}", caller_name, self),
            Err(_) => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        let result =  Ok::<i32, ()>(14);

        result.should_be(14);
        result.should_not_be(12);
        result.should_be_lt(15);
        result.should_be_le(14);
        result.should_be_gt(13);
        result.should_be_ge(14);
        result.should_be_ok();
    }

    #[test]
    #[should_panic(expected = "Err::<i32, ()>(()) should be Ok but was Err(())")]
    fn should_be_ok_fail() {
        Err::<i32, ()>(()).should_be_ok()
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be Err but was Ok(14)")]
    fn should_be_err_fail() {
        Ok::<i32, ()>(14).should_be_err()
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be 12 but was 14")]
    fn should_be_fail() {
        Ok::<i32, ()>(14).should_be(12)
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should not be 14")]
    fn should_not_be_fail() {
        Ok::<i32, ()>(14).should_not_be(14);
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be greater than 15 but was 14")]
    fn should_be_gt_fail() {
        Ok::<i32, ()>(14).should_be_gt(15)
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be greater than or equal to 15 but was 14")]
    fn should_be_ge_fail() {
        Ok::<i32, ()>(14).should_be_ge(15)
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be less than 13 but was 14")]
    fn should_be_lt_fail() {
        Ok::<i32, ()>(14).should_be_lt(13)
    }

    #[test]
    #[should_panic(expected = "Ok::<i32, ()>(14) should be less than or equal to 13 but was 14")]
    fn should_be_le_fail() {
        Ok::<i32, ()>(14).should_be_le(13)
    }
}