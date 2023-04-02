use crate::{ShouldStartWithString, caller_name::get_caller_name};

impl ShouldStartWithString for str {
    fn should_start_with(&self, expected: impl Into<String>) {
        let lhs: String = self.into();
        let rhs: String = expected.into();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if !lhs.starts_with(&rhs) {
            panic!("{} should start with {:?} but was {:?}", caller_name, rhs, lhs)
        }
    }

    fn should_not_start_with(&self, expected: impl Into<String>) {
        let lhs: String = self.into();
        let rhs: String = expected.into();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if lhs.starts_with(&rhs) {
            panic!("{} should not start with {:?} but was {:?}", caller_name, rhs, lhs)
        }
    }

    fn should_end_with(&self, expected: impl Into<String>) {
        let lhs: String = self.into();
        let rhs: String = expected.into();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if !lhs.ends_with(&rhs) {
            panic!("{} should end with {:?} but was {:?}", caller_name, rhs, lhs)
        }
    }

    fn should_not_end_with(&self, expected: impl Into<String>) {
        let lhs: String = self.into();
        let rhs: String = expected.into();
        let caller_name = get_caller_name().unwrap_or("UNKNOWN".to_string());
        if lhs.ends_with(&rhs) {
            panic!("{} should not end with {:?} but was {:?}", caller_name, rhs, lhs)
        }
    }
}