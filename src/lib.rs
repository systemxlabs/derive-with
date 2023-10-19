use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::Token;

#[proc_macro_derive(with)]
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

    let mut constructors = quote!();
    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let constructor_name = format_ident!("with_{}", field_name);

        let constructor = quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn #constructor_name(mut self, #field_name: #field_type) -> Self {
                    self.#field_name = #field_name;
                    self
                }
            }
        };
        constructors = quote! {
            #constructors
            #constructor
        };
    }
    constructors
}

fn with_constructor_for_unnamed(
    ast: &syn::DeriveInput,
    fields: &Punctuated<syn::Field, Token![,]>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let mut constructors = quote!();
    for (index, field) in fields.iter().enumerate() {
        let index = syn::Index::from(index);
        let field_type = &field.ty;
        let param_name = format_ident!("_{}", index);
        let constructor_name = format_ident!("with_{}", index);

        let constructor = quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                pub fn #constructor_name(mut self, #param_name: #field_type) -> Self {
                    self.#index = #param_name;
                    self
                }
            }
        };
        constructors = quote! {
            #constructors
            #constructor
        };
    }
    constructors
}
