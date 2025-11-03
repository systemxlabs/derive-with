use derive_with::With;

#[test]
fn test_simple_struct() {
    #[derive(With, Default)]
    pub struct Foo {
        pub a: i32,
        pub b: String,
    }

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

#[test]
fn test_struct_with_lifetime() {
    #[derive(With, Default)]
    pub struct Foo<'a> {
        pub a: i32,
        pub b: &'a str,
    }

    let foo = Foo::default().with_a(1).with_b("1");
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1");
}

#[test]
fn test_struct_with_generic() {
    #[derive(With, Default)]
    pub struct Foo<T: Default> {
        pub a: i32,
        pub b: T,
    }

    let foo = Foo::<String>::default().with_a(1).with_b("1".to_string());
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1".to_string());
}

#[test]
fn test_struct_with_lifetime_and_generic() {
    #[derive(With, Default)]
    pub struct Foo<'a, T: Default> {
        pub a: T,
        pub b: &'a str,
    }

    let foo = Foo::<i32>::default().with_a(1).with_b("1");
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "1");
}

#[test]
fn test_simple_struct_with_args() {
    #[derive(With, Default)]
    #[with(a)]
    pub struct Foo {
        pub a: i32,
        pub b: String,
    }

    let foo = Foo::default().with_a(1);
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "");
}

#[test]
fn test_struct_switch_generic() {
    #[derive(With, Default)]
    pub struct Foo<T: Default, Z: Default>
    where
        Z: std::fmt::Debug,
    {
        pub a: T,
        pub b: Z,
    }

    let foo = Foo::<String, String>::default().with_a(1);
    assert_eq!(foo.a, 1);
    assert_eq!(foo.b, "");

    let foo = foo.with_b(2);
    assert_eq!(foo.b, 2);
}

#[test]
fn test_single_field_struct() {
    #[derive(With, Default)]
    pub struct SingleField {
        pub single: String,
    }

    let foo = SingleField::default().with_single("a".to_string());
    assert_eq!(&foo.single, "a");
}

#[test]
fn test_struct_with_primitive_types() {
    #[derive(With, Default)]
    pub struct Foo {
        pub a: usize,
    }

    let foo = Foo::default().with_a(1);
    assert_eq!(foo.a, 1);
}
