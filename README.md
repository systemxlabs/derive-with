# A custom derive implementation for `#[derive(with)]`
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/derive-with.svg)](https://crates.io/crates/derive-with)

## Get started
```rust
#[derive(with, Default)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

#[test]
fn test_simple_struct() {
    let foo = Foo::default().with_a(1).with_b(1.to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}
```

## References
- [nrc/derive-new](https://github.com/nrc/derive-new)