# should
`should` is a postfix assertion library for Rust, heavily inspired by [Shouldly](https://docs.shouldly.org/). It aims to make writing assertions feel more natural, while also providing clear error messages.

It does this by generically implementing assertion traits, while also utilizing stack traces to reconstruct the original expression:

```rust
fn multiply(x: i32, y: i32) -> i32 {
    x + y // Oh no a bug!
}

#[test]
fn test_multiply() {
    multiply(3, 5).should_be(15);
}
```
```
panicked at 'multiply(3, 5) should be 15 but was 8'
```

# Assertions
Implemented for `T: PartialEq` as well as `Ok(T)` and `Some(T)`
 - `should_be`
 - `should_not_be`

Implemented for `T: PartialOrd` as well as `Ok(T)` and `Some(T)`
 - `should_be_lt`
 - `should_be_le`
 - `should_be_gt`
 - `should_be_ge`

Implemented for `Option<T>`
 - `should_be_some`
 - `should_be_none`

Implemented for `Result<T, E>`
 - `should_be_ok`
 - `should_be_err`

Implemented for `str`
 - `should_start_with`
 - `should_not_start_with`
 - `should_end_with`
 - `should_not_end_with`

 Implemented for `T: IntoIterator`
 - `should_be_empty`
 - `should_not_be_empty`

All asserted types are required to have implemented the `Debug` trait.