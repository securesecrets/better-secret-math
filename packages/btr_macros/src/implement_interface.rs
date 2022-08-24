use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Type};

const GENERATED_TYPE_SUFFIX: &'static str = "Interface";
const HAS_INTERFACE_ATTRIBUTE: &'static str = "has_interface";
const NO_SHD_ATTRIBUTE: &'static str = "no_shd";
const NO_IMPORT_ATTRIBUTE: &'static str = "no_import";

fn ident_impl(ident: &Ident) -> Ident {
    Ident::new(
        (ident.to_string() + GENERATED_TYPE_SUFFIX).as_ref(),
        Span::call_site(),
    )
}

/// Consumes a type and returns the new type
/// Returns the converted type plus an option detailing which type was replaced if any
fn legacy_conversion(ty: &Type) -> (Type, Option<String>) {
    let mut ty = ty.clone();
    let type_option: Option<String>;
    match ty.clone() {
        Type::Path(mut tp) => {
            let mut path = tp.path.segments.first_mut().unwrap();

            let ident_name = &path.ident.to_string();
            if ident_name == "U256" {
                path.ident = Ident::new("Uint256", Span::call_site());
                type_option = Some(String::from("Uint256"));
            } else if ident_name == "Uint64" {
                path.ident = Ident::new("u64", Span::call_site());
                type_option = Some(String::from("u64"));
            } else if ident_name == "Uint128" {
                path.ident = Ident::new("u128", Span::call_site());
                type_option = Some(String::from("u128"));
            } else {
                type_option = None;
            }
        }
        _ => {
            type_option = None;
        }
    };

    return (ty, type_option);
}

/// Processes the given type and populates the required import (if not already added) and
/// the updated type
fn process_type(item: &Field, types: &mut Vec<Type>, imports: &mut Vec<String>) {
    // Check if were defining an interface
    if item
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(HAS_INTERFACE_ATTRIBUTE))
        .is_some()
    {
        match &item.ty {
            // Replace with its compliant derivative
            Type::Path(tp) => {
                let mut new_tp = tp.clone();
                let mut path = new_tp.path.segments.first_mut().unwrap();
                path.ident = ident_impl(&path.ident);

                types.push(Type::Path(new_tp));
            }
            _ => panic!("Attribute must be a struct"),
        }
    } else {
        // Try to convert and add its relevant imports
        let (converted, import) = legacy_conversion(&item.ty);
        if let Some(import) = import {
            if !imports.contains(&import) {
                imports.push(import);
            }
        }
        types.push(converted);
    }
}

pub(crate) fn impl_support_interface(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let impl_name = ident_impl(&name);

    // TODO: add support for none, pub, pub(crate) for parameters and the struct
    // TODO: implement From<name> for impl_name

    let mut imports = vec![];

    let mut no_shd = false;
    let mut no_import = false;

    // Process relevant struct attributes
    if ast
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(NO_IMPORT_ATTRIBUTE))
        .is_some()
    {
        no_import = true;
    } else if ast
        .attrs
        .iter()
        .find(|attribute| attribute.path.is_ident(NO_SHD_ATTRIBUTE))
        .is_some()
    {
        no_shd = true;
    }

    let mut generated = match ast.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                // struct parameter names
                let mut names = vec![];
                // struct parameter types
                let mut types = vec![];

                for item in named.iter() {
                    process_type(item, &mut types, &mut imports);
                    names.push(item.ident.clone().unwrap());
                }

                quote!(
                    pub struct #impl_name {
                        #(
                            pub #names: #types
                        ),*
                    }
                )
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                //TODO: implement support for unnamed newtypes
                // struct parameter types
                let mut types = vec![];

                for item in unnamed.iter() {
                    process_type(item, &mut types, &mut imports);
                }

                panic!("Newtypes not implemented")
            }
            Fields::Unit => panic!("Units not implemented"),
        },
        Data::Enum(_) => panic!("Enum not implemented"), //TODO: implement support for enums
        Data::Union(_) => panic!("Union not implemented"),
    };

    let mut import_token = quote!();
    if !no_import {
        if !imports.is_empty() {
            if no_shd {
                import_token = quote!(
                    use shade_protocol::c_std::{
                        #(#imports),*
                    }
                )
            } else {
                import_token = quote!(
                    use cosmwasm_std::{
                        #(#imports),*
                    }
                )
            }
        }
    }

    generated = quote!(
        #import_token

        #generated
    );

    TokenStream::from(generated)
}
