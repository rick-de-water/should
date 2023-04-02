use crate::ShouldStartWithString;

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