use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Attribute, Index, Meta, Token};

/// A custom derive implementation for `#[derive(with)]`
///
/// # Get started
///
/// 1.Generate with constructor for each field
/// ```rust
/// use derive_with::with;
///
/// #[derive(with, Default)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(with, Default)]
/// pub struct Bar (i32, String);
///
/// #[test]
/// fn test_struct() {
///     let foo = Foo::default().with_a(1).with_b(1.to_string());
///     assert_eq!(foo.a, 1);
///     assert_eq!(foo.b, "1".to_string());
///
///     let bar = Bar::default().with_0(1).with_1(1.to_string());
///     assert_eq!(bar.0, 1);
///     assert_eq!(bar.1, "1".to_string());
/// }
/// ```
///
/// 2.Generate with constructor for specific fields
/// ```rust
/// use derive_with::with;
///
/// #[derive(with, Default)]
/// #[with(a)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(with, Default)]
/// #[with(1)]
/// pub struct Bar (i32, String);
///
/// #[test]
/// fn test_struct() {
///     let foo = Foo::default().with_a(1);
///     assert_eq!(foo.a, 1);
///
///     let bar = Bar::default().with_1(1.to_string());
///     assert_eq!(bar.1, "1".to_string());
/// }
/// ```
#[proc_macro_derive(with, attributes(with))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Couldn't parse item");
    let result = match ast.data {
        syn::Data::Struct(ref s) => with_for_struct(&ast, &s.fields),
        syn::Data::Enum(_) => panic!("doesn't work with enums yet"),
        syn::Data::Union(_) => panic!("doesn't work with unions yet"),
    };
    result.into()
}

fn with_for_struct(ast: &syn::DeriveInput, fields: &syn::Fields) -> proc_macro2::TokenStream {
    match *fields {
        syn::Fields::Named(ref fields) => with_constructor_for_named(ast, &fields.named),
        syn::Fields::Unnamed(ref fields) => with_constructor_for_unnamed(ast, &fields.unnamed),
        syn::Fields::Unit => panic!("Unit structs are not supported"),
    }
}

fn with_constructor_for_named(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let with_args = parse_with_args::<Ident>(&ast.attrs);

    let mut constructors = quote!();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if !contains_field(&with_args, field_name) {
            continue;
        }
        let field_type = &field.ty;
        let constructor_name = format_ident!("with_{}", field_name);

        let constructor = quote! {
            pub fn #constructor_name(mut self, #field_name: impl Into<#field_type>) -> Self {
                self.#field_name = #field_name.into();
                self
            }
        };
        constructors = quote! {
            #constructors
            #constructor
        };
    }
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #constructors
        }
    }
}

fn with_constructor_for_unnamed(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let with_args = parse_with_args::<Index>(&ast.attrs);

    let mut constructors = quote!();
    for (index, field) in fields.iter().enumerate() {
        let index = syn::Index::from(index);
        if !contains_field(&with_args, &index) {
            continue;
        }
        let field_type = &field.ty;
        let param_name = format_ident!("field_{}", index);
        let constructor_name = format_ident!("with_{}", index);

        let constructor = quote! {
            pub fn #constructor_name(mut self, #param_name: impl Into<#field_type>) -> Self {
                self.#index = #param_name.into();
                self
            }
        };
        constructors = quote! {
            #constructors
            #constructor
        };
    }
    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #constructors
        }
    }
}

fn parse_with_args<T: Parse>(attrs: &Vec<Attribute>) -> Option<Punctuated<T, Comma>> {
    if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident("with")) {
        match &attr.meta {
            Meta::List(list) => Some(
                list.parse_args_with(Punctuated::<T, Comma>::parse_terminated)
                    .expect("Couldn't parse with args"),
            ),
            _ => panic!("`with` attribute should like `#[with(a, b, c)]`"),
        }
    } else {
        None
    }
}

fn contains_field<T: Parse + PartialEq>(
    with_args: &Option<Punctuated<T, Comma>>,
    item: &T,
) -> bool {
    with_args.is_none() || with_args.as_ref().unwrap().iter().any(|arg| arg == item)
}
