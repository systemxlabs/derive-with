# A custom derive implementation for `#[derive(with)]`
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/derive-with.svg)](https://crates.io/crates/derive-with)

## Get started

1.Generate with constructor for each field
```rust
use derive_with::with;

#[derive(with, Default)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

#[derive(with, Default)]
pub struct Bar (i32, String);

#[test]
fn test_struct() {
    let foo = Foo::default().with_a(1).with_b(1.to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
    
    let bar = Bar::default().with_0(1).with_1(1.to_string());
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}
```

2.Generate with constructor for specific fields
```rust
#[derive(with, Default)]
#[with(a)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

#[derive(with, Default)]
#[with(1)]
pub struct Bar (i32, String);

#[test]
fn test_struct() {
    let foo = Foo::default().with_a(1);
    assert_eq!(foo.a, 1);

    let bar = Bar::default().with_1(1.to_string());
    assert_eq!(bar.1, "1".to_string());
}
```

## References
- [nrc/derive-new](https://github.com/nrc/derive-new)