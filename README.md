# A custom derive implementation for `#[derive(new)]`

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