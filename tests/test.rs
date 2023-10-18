use derive_with::with;

#[derive(with, PartialEq, Debug)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}
impl Foo {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: "0".to_string(),
        }
    }
}

#[test]
fn test_simple_struct() {
    let foo = Foo::new().with_a(1).with_b(1.to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}
