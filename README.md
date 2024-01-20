# A custom derive implementation for `#[derive(With)]`
![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Crates.io](https://img.shields.io/crates/v/derive-with.svg)](https://crates.io/crates/derive-with)

## Get started

1.Generate with-constructor for each field on named struct.
```rust
#[derive(With)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}
```
This will generate code
```rust
impl Foo {
    pub fn with_a(mut self, a: impl Into<i32>) -> Self {
        self.a = a.into();
        self
    }
    pub fn with_b(mut self, b: impl Into<String>) -> Self {
        self.b = b.into();
        self
    }
}
```

2.Generate with-constructor for each field on tuple struct.
```rust
#[derive(With)]
pub struct Bar (i32, String);
```
This will generate code
```rust
impl Bar {
    pub fn with_0(mut self, field_0: impl Into<i32>) -> Self {
        self.0 = field_0.into();
        self
    }
    pub fn with_1(mut self, field_1: impl Into<String>) -> Self {
        self.1 = field_1.into();
        self
    }
}
```

3.Generate with-constructor for specific fields on named struct.
```rust
#[derive(With)]
#[with(a)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}
```
This will generate code
```rust
impl Foo {
    pub fn with_a(mut self, a: impl Into<i32>) -> Self {
        self.a = a.into();
        self
    }
}
```

4.Generate with-constructor for specific fields on tuple struct.
```rust
#[derive(With)]
#[with(1)]
pub struct Bar (i32, String);
```
This will generate code
```rust
impl Bar {
    pub fn with_1(mut self, field_1: impl Into<String>) -> Self {
        self.1 = field_1.into();
        self
    }
}
```

## References
- [nrc/derive-new](https://github.com/nrc/derive-new)