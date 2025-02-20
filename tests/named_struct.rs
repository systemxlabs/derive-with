use derive_with::With;

#[derive(With, Default)]
pub struct Foo {
    pub a: i32,
    pub b: String,
}

#[test]
fn test_simple_struct() {
    struct MyString(pub String);
    impl From<MyString> for String {
        fn from(value: MyString) -> Self {
            value.0
        }
    }

    let foo = Foo::default().with_a(1).with_b(MyString("1".to_string()));
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}

#[derive(With, Default)]
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

#[derive(With, Default)]
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

#[derive(With, Default)]
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

#[derive(With, Default)]
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

#[derive(With, Default)]
pub struct Foo5<T: Default, Z: Default>
where
    Z: std::fmt::Debug,
{
    pub a: T,
    pub b: Z,
}

#[test]
fn test_struct_switch_generic() {
    let foo = Foo5::<String, String>::default().with_a(1);
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "");

    let foo = foo.with_b(2);
    assert_eq!(foo.b, 2);
}
