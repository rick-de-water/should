use crate::{ShouldStartEndString, caller_name::get_caller_name};

impl ShouldStartEndString for str {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success() {
        "foobar".should_start_with("foo");
        "foobar".to_string().should_start_with("foo");
        "foobar".should_start_with("foo".to_string());
        "foobar".to_string().should_start_with("foo".to_string());
        
        "foobar".should_not_start_with("bar");
        "foobar".to_string().should_not_start_with("bar");
        "foobar".should_not_start_with("bar".to_string());
        "foobar".to_string().should_not_start_with("bar".to_string());

        "foobar".should_end_with("bar");
        "foobar".to_string().should_end_with("bar");
        "foobar".should_end_with("bar".to_string());
        "foobar".to_string().should_end_with("bar".to_string());
        
        "foobar".should_not_end_with("foo");
        "foobar".to_string().should_not_end_with("foo");
        "foobar".should_not_end_with("foo".to_string());
        "foobar".to_string().should_not_end_with("foo".to_string());
    }

    #[test]
    #[should_panic(expected = r#""foobar" should start with "bar" but was "foobar""#)]
    fn should_start_with_fail() {
        "foobar".should_start_with("bar");
    }

    #[test]
    #[should_panic(expected = r#""foobar" should not start with "foo" but was "foobar""#)]
    fn should_not_start_with_fail() {
        "foobar".should_not_start_with("foo");
    }

    #[test]
    #[should_panic(expected = r#""foobar" should end with "foo" but was "foobar""#)]
    fn should_end_with_fail() {
        "foobar".should_end_with("foo");
    }

    #[test]
    #[should_panic(expected = r#""foobar" should not end with "bar" but was "foobar""#)]
    fn should_not_end_with_fail() {
        "foobar".should_not_end_with("bar");
    }
}