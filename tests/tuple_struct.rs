use derive_with::With;

#[test]
fn test_simple_tuple_struct() {
    #[derive(With, Default)]
    pub struct Bar(i32, String);

    struct MyString(pub String);
    impl From<MyString> for String {
        fn from(value: MyString) -> Self {
            value.0
        }
    }

    let bar = Bar::default().with_0(1).with_1(MyString("1".to_string()));
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}

#[test]
fn test_tuple_struct_with_lifetime() {
    #[derive(With, Default)]
    pub struct Bar<'a>(i32, &'a str);

    let bar = Bar::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[test]
fn test_tuple_struct_with_generic() {
    #[derive(With, Default)]
    pub struct Bar<T: Default>(i32, T);

    let bar = Bar::<String>::default().with_0(1).with_1("1".to_string());
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}

#[test]
fn test_tuple_struct_with_lifetime_and_generic() {
    #[derive(With, Default)]
    pub struct Bar<'a, T: Default>(T, &'a str);

    let bar = Bar::<i32>::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[test]
fn test_simple_tuple_struct_with_args() {
    #[derive(With, Default)]
    #[with(1)]
    pub struct Bar(i32, String);

    let bar = Bar::default().with_1("1");
    assert_eq!(bar.0, 0);
    assert_eq!(bar.1, "1");
}

#[test]
fn test_tuple_struct_switch_generic() {
    #[derive(With, Default)]
    pub struct Bar<T: Default, Z: Default>(T, Z)
    where
        Z: std::fmt::Debug;

    let bar = Bar::<String, String>::default().with_0(1);
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "");

    let bar = bar.with_1(2);
    assert_eq!(bar.1, 2);
}

#[test]
fn test_tuple_struct_with_primitive_types() {
    #[derive(With, Default)]
    pub struct Bar(usize);

    let bar = Bar::default().with_0(1);
    assert_eq!(bar.0, 1);
}
