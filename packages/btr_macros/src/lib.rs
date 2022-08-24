use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

mod btr;
mod implement_interface;

#[proc_macro_derive(support_interface, attributes(has_interface))]
pub fn support_interface(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    implement_interface::impl_support_interface(ast)
}

#[proc_macro_derive(Btr)]
pub fn btr_struct(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let expanded = btr::btr_struct(input).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn btr_derive(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = parse_macro_input!(attr as Ident);
    let expanded = btr::derive(input, ident).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn btr_serde(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = btr::serde_impl(input).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}
