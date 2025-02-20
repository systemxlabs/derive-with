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
#[automatically_derived]
impl Foo {
    pub fn with_a(self, a: impl Into<i32>) -> Self {
        Self {
            a: a.into(),
            ..self
        }
    }
    pub fn with_b(self, b: impl Into<String>) -> Self {
        Self {
            b: b.into(),
            ..self
        }
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
#[automatically_derived]
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
#[automatically_derived]
impl Foo {
    pub fn with_a(self, a: impl Into<i32>) -> Self {
        Self {
            a: a.into(),
            ..self
        }
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
#[automatically_derived]
impl Bar {
    pub fn with_1(mut self, field_1: impl Into<String>) -> Self {
        self.1 = field_1.into();
        self
    }
}
```

5.Generate with-constructor for generic fields
```rust
#[derive(With, Default)]
pub struct Foo<T: Default, Z: Default>
where
    Z: std::fmt::Debug,
{
    pub a: T,
    pub b: Z,
}
```
This will generate code
```rust
#[automatically_derived]
impl<T: Default, Z: Default> Foo<T, Z>
where
    Z: std::fmt::Debug,
{
    pub fn with_a<WT: Default>(self, a: WT) -> Foo<WT, Z> {
        Foo { a, b: self.b }
    }
    pub fn with_b<WZ: Default>(self, b: WZ) -> Foo<T, WZ>
    where
        WZ: std::fmt::Debug,
    {
        Foo { a: self.a, b }
    }
}
```

More examples can be found in [tests](./tests/)

## References
- [nrc/derive-new](https://github.com/nrc/derive-new)