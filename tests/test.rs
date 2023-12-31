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

#[derive(with, Default)]
pub struct Foo3<'a, T: Default> {
    pub a: T,
    pub b: &'a str,
}
#[test]
fn test_struct_with_lifetime_and_generic() {
    let foo = Foo3::<i32>::default().with_a(1).with_b("1");
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1");
}

#[derive(with, Default)]
#[with(a)]
pub struct Foo4 {
    pub a: i32,
    pub b: String,
}
#[test]
fn test_simple_struct_with_args() {
    let foo = Foo4::default().with_a(1);
    assert_eq!(foo.a, 1);
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

#[derive(with, Default)]
pub struct Bar3<'a, T: Default>(T, &'a str);
#[test]
fn test_tuple_struct_with_lifetime_and_generic() {
    let bar = Bar3::<i32>::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[derive(with, Default)]
#[with(1)]
pub struct Bar4(i32, String);
#[test]
fn test_simple_tuple_struct_with_args() {
    let bar = Bar4::default().with_1(1);
    assert_eq!(bar.0, 1);
}
