use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{ToTokens, format_ident, quote};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    Attribute, GenericParam, Generics, Index, Meta, Path, PredicateType, Token, Type, TypeParam,
    TypePath, WhereClause, WherePredicate,
};

/// A custom derive implementation for `#[derive(With)]`
///
/// # Get started
///
/// 1.Generate with-constructor for each field
/// ```rust
/// use derive_with::With;
///
/// #[derive(With, Default)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(With, Default)]
/// pub struct Bar (i32, String);
///
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
/// 2.Generate with-constructor for specific fields
/// ```rust
/// use derive_with::With;
///
/// #[derive(With, Default)]
/// #[with(a)]
/// pub struct Foo {
///     pub a: i32,
///     pub b: String,
/// }
///
/// #[derive(With, Default)]
/// #[with(1)]
/// pub struct Bar (i32, String);
///
/// fn test_struct() {
///     let foo = Foo::default().with_a(1);
///     assert_eq!(foo.a, 1);
///
///     let bar = Bar::default().with_1(1.to_string());
///     assert_eq!(bar.1, "1".to_string());
/// }
/// ```
#[proc_macro_derive(With, attributes(with))]
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
    let generics_map = index_generics(&ast.generics);
    let where_predicate_map = index_where_predicates(&ast.generics.where_clause);
    let with_args = parse_with_args::<Ident>(&ast.attrs);
    let field_count = fields.len();

    let mut constructors = quote!();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        if !contains_field(&with_args, field_name) {
            continue;
        }
        let field_type = &field.ty;
        let constructor_name = format_ident!("with_{}", field_name);

        // Check the type of the field
        let constructor = match field_type {
            // For simple path types
            Type::Path(type_path) => {
                // Check if the type matches some generic parameter
                match generics_map.get(&type_path.path).cloned() {
                    // If the type is not generic, just use the Into trait to derive the method
                    None => generate_constructor_for_named(
                        &constructor_name,
                        field_name,
                        field_type,
                        field_count,
                    ),
                    // If the type is generic, allow to switch types
                    Some(mut generic) => {
                        let new_generic = format_ident!("W{}", generic.ident);
                        // Update the generic ident for the new one, so that it doesn't conflict with the existing
                        generic.ident = new_generic.clone();

                        // Determine the new generics, which are the existing generics
                        let mut new_generic_params = Vec::new();
                        for param in &ast.generics.params {
                            new_generic_params.push(match param {
                                // Except for the generic parameter that matches the field type
                                GenericParam::Type(type_param)
                                    if type_path.path.is_ident(&type_param.ident) =>
                                {
                                    // That must be replaced with the new generic ident
                                    new_generic.to_token_stream()
                                }
                                GenericParam::Type(type_param) => {
                                    type_param.ident.to_token_stream()
                                }
                                GenericParam::Lifetime(lifetime_param) => {
                                    lifetime_param.lifetime.to_token_stream()
                                }
                                GenericParam::Const(const_param) => {
                                    const_param.ident.to_token_stream()
                                }
                            });
                        }

                        // Compute the new field values, as we can't deconstruct when switching types
                        let mut other_fields = Vec::new();
                        for other_field in fields {
                            let other_field_name = other_field.ident.as_ref().unwrap();
                            if other_field_name != field_name {
                                other_fields
                                    .push(quote! { #other_field_name: self.#other_field_name });
                            } else {
                                other_fields.push(quote! { #field_name });
                            }
                        }

                        // Retrieve the where predicate affecting this field, if any
                        let where_clause = where_predicate_map.get(&type_path.path).cloned().map(
                            |mut predicate| {
                                // And update the bounded type to the new generic ident
                                predicate.bounded_ty = Type::Path(TypePath {
                                    qself: None,
                                    path: Path::from(new_generic.clone()),
                                });
                                quote! { where #predicate }
                            },
                        );

                        quote! {
                            pub fn #constructor_name <#generic> (self, #field_name: #new_generic)
                            -> #name < #(#new_generic_params),* >
                            #where_clause
                            {
                                #name {
                                    #(#other_fields),*
                                }
                            }
                        }
                    }
                }
            }
            // For every other field type, just use the Into trait to derive the method
            _ => generate_constructor_for_named(
                &constructor_name,
                field_name,
                field_type,
                field_count,
            ),
        };

        constructors = quote! {
            #constructors
            #constructor
        };
    }
    quote! {
        #[automatically_derived]
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
    let generics_map = index_generics(&ast.generics);
    let where_predicate_map = index_where_predicates(&ast.generics.where_clause);
    let with_args = parse_with_args::<Index>(&ast.attrs);

    let mut constructors = quote!();
    for (index, field) in fields.iter().enumerate() {
        let index = syn::Index::from(index);
        if !contains_field(&with_args, &index) {
            continue;
        }
        let field_type = &field.ty;
        let field_name = format_ident!("field_{}", index);
        let constructor_name = format_ident!("with_{}", index);

        // Check the type of the field
        let constructor = match field_type {
            // For simple path types
            Type::Path(type_path) => {
                // Check if the type matches some generic parameter
                match generics_map.get(&type_path.path).cloned() {
                    // If the type is not generic, just use the Into trait to derive the method
                    None => {
                        quote! {
                            pub fn #constructor_name(mut self, #field_name: impl Into<#field_type>) -> Self {
                                self.#index = #field_name.into();
                                self
                            }
                        }
                    }
                    // If the type is generic, allow to switch types
                    Some(mut generic) => {
                        let new_generic = format_ident!("W{}", generic.ident);
                        // Update the generic ident for the new one, so that it doesn't conflict with the existing
                        generic.ident = new_generic.clone();

                        // Determine the new generics, which are the existing generics
                        let mut new_generic_params = Vec::new();
                        for param in &ast.generics.params {
                            new_generic_params.push(match param {
                                // Except for the generic parameter that matches the field type
                                GenericParam::Type(type_param)
                                    if type_path.path.is_ident(&type_param.ident) =>
                                {
                                    // That must be replaced with the new generic ident
                                    new_generic.to_token_stream()
                                }
                                GenericParam::Type(type_param) => {
                                    type_param.ident.to_token_stream()
                                }
                                GenericParam::Lifetime(lifetime_param) => {
                                    lifetime_param.lifetime.to_token_stream()
                                }
                                GenericParam::Const(const_param) => {
                                    const_param.ident.to_token_stream()
                                }
                            });
                        }

                        // Compute the new field values
                        let mut other_fields = Vec::new();
                        for (other_index, _) in fields.iter().enumerate() {
                            let other_index = syn::Index::from(other_index);
                            if other_index != index {
                                other_fields.push(quote! { self.#other_index });
                            } else {
                                other_fields.push(quote! { #field_name });
                            }
                        }

                        // Retrieve the where predicate affecting this field, if any
                        let where_clause = where_predicate_map.get(&type_path.path).cloned().map(
                            |mut predicate| {
                                // And update the bounded type to the new generic ident
                                predicate.bounded_ty = Type::Path(TypePath {
                                    qself: None,
                                    path: Path::from(new_generic.clone()),
                                });
                                quote! { where #predicate }
                            },
                        );

                        quote! {
                            pub fn #constructor_name <#generic> (self, #field_name: #new_generic)
                            -> #name < #(#new_generic_params),* >
                            #where_clause
                            {
                                #name ( #(#other_fields),* )
                            }
                        }
                    }
                }
            }
            // For every other field type, just use the Into trait to derive the method
            _ => {
                quote! {
                    pub fn #constructor_name(mut self, #field_name: impl Into<#field_type>) -> Self {
                        self.#index = #field_name.into();
                        self
                    }
                }
            }
        };

        constructors = quote! {
            #constructors
            #constructor
        };
    }
    quote! {
        #[automatically_derived]
        impl #impl_generics #name #ty_generics #where_clause {
            #constructors
        }
    }
}

fn parse_with_args<T: Parse>(attrs: &[Attribute]) -> Option<Punctuated<T, Comma>> {
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

fn index_generics(generics: &Generics) -> HashMap<Path, TypeParam> {
    generics
        .params
        .iter()
        .filter_map(|p| match p {
            GenericParam::Type(type_param) => Some(type_param),
            _ => None,
        })
        .map(|p| (Path::from(p.ident.clone()), p.clone()))
        .collect()
}

fn index_where_predicates(where_clause: &Option<WhereClause>) -> HashMap<Path, PredicateType> {
    where_clause
        .as_ref()
        .map(|w| {
            w.predicates
                .iter()
                .filter_map(|p| match p {
                    WherePredicate::Type(t) => Some(t),
                    _ => None,
                })
                .filter_map(|t| match &t.bounded_ty {
                    Type::Path(type_path) => Some((type_path.path.clone(), t.clone())),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn generate_constructor_for_named(
    constructor_name: &Ident,
    field_name: &Ident,
    field_type: &Type,
    field_count: usize,
) -> proc_macro2::TokenStream {
    if field_count == 1 {
        quote! {
            pub fn #constructor_name(self, #field_name: impl Into<#field_type>) -> Self {
                Self {
                    #field_name: #field_name.into(),
                }
            }
        }
    } else {
        quote! {
            pub fn #constructor_name(self, #field_name: impl Into<#field_type>) -> Self {
                Self {
                    #field_name: #field_name.into(),
                    ..self
                }
            }
        }
    }
}
