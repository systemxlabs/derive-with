use derive_with::with;

/// Named Struct Tests
#[derive(with, Default)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}
#[test]
fn test_simple_struct() {
    let foo = Foo::default().with_a(1).with_b("1".to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}

#[derive(with, Default)]
pub struct Foo1<'a> {
    pub a: i32,
    pub b: &'a str,
}
#[test]
fn test_struct_with_lifetime() {
    let foo = Foo1::default().with_a(1).with_b("1");
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1");
}

#[derive(with, Default)]
pub struct Foo2<T: Default> {
    pub a: i32,
    pub b: T,
}
#[test]
fn test_struct_with_generic() {
    let foo = Foo2::<String>::default().with_a(1).with_b("1".to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}

/// Tuple Struct Tests
#[derive(with, Default)]
pub struct Bar(i32, String);
#[test]
fn test_simple_tuple_struct() {
    let bar = Bar::default().with_0(1).with_1("1".to_string());
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}

#[derive(with, Default)]
pub struct Bar1<'a>(i32, &'a str);
#[test]
fn test_tuple_struct_with_lifetime() {
    let bar = Bar1::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[derive(with, Default)]
pub struct Bar2<T: Default>(i32, T);
#[test]
fn test_tuple_struct_with_generic() {
    let bar = Bar2::<String>::default().with_0(1).with_1("1".to_string());
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}
