# should
[![crates.io](https://img.shields.io/crates/v/should)](https://crates.io/crates/should)
[![docs](https://img.shields.io/docsrs/should)](https://docs.rs/should)
[![msrv](https://img.shields.io/crates/msrv/should)](https://docs.rs/should)

should is a postfix assertion library for Rust, heavily inspired by [Shouldly](https://docs.shouldly.org/). It aims to make writing assertions feel more natural, while also providing clearer error messages.

```rust
use should::*;

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

## How does it work?
should defines a set of assertion traits, which it implements generically for most types. This is what enables the postfix syntax.

When the assertion fails, should uses a stacktrace to reconstruct the original expression. It finds the file and line where should was called, parses the original expression, and uses that information to generate a nice panic message.

This does however mean that builds without debug symbols (e.g. release build by default) are not able to retrieve the expression. A placeholder will be used for the expression when this is the case, and everything else should still work as expected.

## License
Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.