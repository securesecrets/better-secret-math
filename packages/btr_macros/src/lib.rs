use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

mod borsh;
mod btr;
mod implement_interface;

/// has_interface replaces the given type with a support_interface compliant version
/// no_shd replaces the library imports so it works with standard cosmwasm imports
/// no_import skips the import generation (useful if using other custom equivalents)
#[proc_macro_derive(support_interface, attributes(has_interface, no_shd, no_import))]
pub fn support_interface(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    implement_interface::impl_support_interface(ast)
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

#[proc_macro_attribute]
pub fn borsh_derive(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = parse_macro_input!(attr as Ident);
    let expanded = borsh::derive(input, ident).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn borsh_serde(
    _attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = borsh::serde_impl(input).into_token_stream();

    proc_macro::TokenStream::from(expanded)
}
