use derive_with::With;

#[derive(With, Default)]
pub struct Bar(i32, String);

#[test]
fn test_simple_tuple_struct() {
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

#[derive(With, Default)]
pub struct Bar1<'a>(i32, &'a str);

#[test]
fn test_tuple_struct_with_lifetime() {
    let bar = Bar1::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[derive(With, Default)]
pub struct Bar2<T: Default>(i32, T);

#[test]
fn test_tuple_struct_with_generic() {
    let bar = Bar2::<String>::default().with_0(1).with_1("1".to_string());
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1".to_string());
}

#[derive(With, Default)]
pub struct Bar3<'a, T: Default>(T, &'a str);

#[test]
fn test_tuple_struct_with_lifetime_and_generic() {
    let bar = Bar3::<i32>::default().with_0(1).with_1("1");
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "1");
}

#[derive(With, Default)]
#[with(1)]
pub struct Bar4(i32, String);

#[test]
fn test_simple_tuple_struct_with_args() {
    let bar = Bar4::default().with_1("1");
    assert_eq!(bar.1, "1");
}

#[derive(With, Default)]
pub struct Bar5<T: Default, Z: Default>(T, Z)
where
    Z: std::fmt::Debug;

#[test]
fn test_tuple_struct_switch_generic() {
    let bar = Bar5::<String, String>::default().with_0(1);
    assert_eq!(bar.0, 1);
    assert_eq!(bar.1, "");

    let bar = bar.with_1(2);
    assert_eq!(bar.1, 2);
}
