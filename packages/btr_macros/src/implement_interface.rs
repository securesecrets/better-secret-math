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
/// // TODO: option string could be replaced with a mut vec of unique tokens
fn legacy_conversion(ty: &Type) -> (Type, Option<String>) {
    let mut ty = ty.clone();
    let type_option: Option<String>;
    match ty.clone() {
        Type::Path(mut tp) => {
            let path = tp.path.segments.first_mut().unwrap();

            let ident_name = &path.ident.to_string();
            if ident_name == "U256" {
                path.ident = Ident::new("Uint256", Span::call_site());
                type_option = Some(String::from("Uint256"));
            } else if ident_name == "u64" {
                path.ident = Ident::new("Uint64", Span::call_site());
                type_option = Some(String::from("Uint64"));
            } else if ident_name == "u128" {
                path.ident = Ident::new("Uint128", Span::call_site());
                type_option = Some(String::from("Uint128"));
            } else {
                type_option = None;
            }
            ty = Type::Path(tp);
        }
        _ => {
            type_option = None;
        }
    };

    return (ty, type_option);
}

fn from_field_conversion(
    posfix: proc_macro2::TokenStream,
    ty: &Type,
    has_flag: bool,
) -> proc_macro2::TokenStream {
    match ty.clone() {
        Type::Path(mut tp) => {
            let path = tp.path.segments.first_mut().unwrap();

            let type_name = &path.ident.to_string();
            if type_name == "Uint256" || has_flag {
                quote!(
                    x.#posfix.into()
                )
            } else if type_name == "Uint64" {
                quote!(
                    Uint64::new(x.#posfix)
                )
            } else if type_name == "Uint128" {
                quote!(
                    Uint128::new(x.#posfix)
                )
            } else {
                quote!(
                    x.#posfix
                )
            }
        }
        _ => {
            panic!("Field conversion only valid for path");
        }
    }
}

/// Processes the given type and populates the required import (if not already added) and
/// the updated type
///
/// Returns the new type and if its has an interface attribute
fn process_type(item: &Field, types: &mut Vec<Type>, imports: &mut Vec<String>) -> (Type, bool) {
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
                let new_type = Type::Path(new_tp);
                types.push(new_type.clone());
                return (new_type, true);
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
        let new_type = converted.clone();
        types.push(converted);
        return (new_type, false);
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
                // From conversion
                let mut from_conversion = vec![];

                for item in named.iter() {
                    let (new_type, has_flag) = process_type(item, &mut types, &mut imports);
                    let item_name = item.ident.clone().unwrap();
                    from_conversion.push(from_field_conversion(
                        quote!(#item_name),
                        &new_type,
                        has_flag,
                    ));
                    names.push(item_name);
                }

                quote!(
                    pub struct #impl_name {
                        #(
                            pub #names: #types
                        ),*
                    }

                    impl From<#name> for #impl_name {
                        fn from(x: #name) -> Self {
                            Self {
                                #(#names: #from_conversion),*
                            }
                        }
                    }
                )
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let mut types = vec![];
                //let mut from_conversion = vec![];

                for item in unnamed.iter() {
                    process_type(item, &mut types, &mut imports);
                }

                quote!(
                    pub struct #impl_name (
                        #(
                            #types
                        ),*
                    );
                )
            }
            Fields::Unit => panic!("Units not implemented"),
        },
        Data::Enum(_) => panic!("Enum not implemented"), //TODO: implement support for enums
        Data::Union(_) => panic!("Union not implemented"),
    };

    // Doing this the ugly way cause I cant find a solution
    let mut import_token = quote!();
    if !no_import {
        if !imports.is_empty() {
            if !no_shd {
                for import in imports {
                    if import == "Uint256".to_string() {
                        import_token = quote!(
                            #import_token
                            use shade_protocol::c_std::Uint256;
                        )
                    } else if import == "Uint128".to_string() {
                        import_token = quote!(
                            #import_token
                            use shade_protocol::c_std::Uint128;
                        )
                    } else if import == "Uint64".to_string() {
                        import_token = quote!(
                            #import_token
                            use shade_protocol::c_std::Uint64;
                        )
                    } else {
                        panic!("Import not defined {}", import.to_string())
                    }
                }
            } else {
                for import in imports {
                    if import == "Uint256".to_string() {
                        import_token = quote!(
                            #import_token
                            use cosmwasm_std::Uint256;
                        )
                    } else if import == "Uint128".to_string() {
                        import_token = quote!(
                            #import_token
                            use cosmwasm_std::Uint128;
                        )
                    } else if import == "Uint64".to_string() {
                        import_token = quote!(
                            #import_token
                            use cosmwasm_std::Uint64;
                        )
                    } else {
                        panic!("Import not defined {}", import.to_string())
                    }
                }
            }
        }
    }

    generated = quote!(
        #import_token

        #generated
    );

    TokenStream::from(generated)
}
