use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{parse_quote, DeriveInput, Data, DataStruct, Fields, Type, TypePath, Path};

pub fn convert_to_btr(ty: &Type) -> Type {
    match ty {
        Type::Path(type_path) => {
            let u256_ty = Type::Path(TypePath {
                qself: None,
                path: Path::from(Ident::new("U256", Span::call_site())),
            });
            let addr_ty = Type::Path(TypePath {
                qself: None,
                path: Path::from(Ident::new("Addr", Span::call_site())),
            });
            let u64_ty = Type::Path(TypePath {
                qself: None,
                path: Path::from(Ident::new("u64", Span::call_site())),
            });
            let u128_ty = Type::Path(TypePath {
                qself: None,
                path: Path::from(Ident::new("u128", Span::call_site())),
            });
            let path = &type_path.path;
            if path.is_ident("Addr") {
                addr_ty
            } else if path.is_ident("Uint128") {
                u128_ty
            } else if path.is_ident("Uint64") {
                u64_ty
            } else if path.is_ident("Decimal256") || path.is_ident("Uint256") {
                u256_ty
            } else {
                let mut btr = "Btr".to_string();
                btr.push_str(&path.into_token_stream().to_string());
                Type::Path(TypePath {
                    qself: None,
                    path: Path::from(Ident::new(btr.as_str(), Span::call_site())),
                })
            }
        },
        _ => panic!("Unsupported type."),
    }
}

pub fn btr_struct(input: DeriveInput) -> DeriveInput {
    let fields = match &input.data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => &fields.named,
        _ => panic!("expected a struct with named fields"),
    };
    let element = fields.iter().map(|field| &field.ident);
    let ty = fields.iter().map(|field| &field.ty);
    let btr_ty = fields.iter().map(|field| convert_to_btr(&field.ty) );
    let attrs = fields.iter().map(|field| &field.attrs);
    let struct_name = &input.ident;

    parse_quote! {
        paste::paste! {
            #[btr_macros::btr_derive([<Btr #struct_name>])]
            #input
            // #crate::impl_new! {
            //     #struct {
            //         #(#element, #ty); *
            //     }
            // }
            #[doc = "[" #struct_name "] optimized for math and storage (via support for Bincode2 serialization)."]
            #[btr_macros::btr_derive(#struct_name)]
            pub struct [<Btr #struct_name>] {
                #(
                    pub #element: #btr_ty
                ),*
            }
            // #crate::impl_new! {
            //     [<Btr #struct>] {
            //         #(#element, #btr_ty); *
            //     }
            // }
        }
    }
}

pub fn derive(input: DeriveInput, ident: Ident) -> DeriveInput {
    let name = input.ident.to_string();

    if !name.contains("Btr") {
        match input.data {
            syn::Data::Struct(_) => parse_quote! {
                #[cosmwasm_schema::cw_serde]
                #[derive(derive_from_ext::From)]
                #[from(#ident)]
                #input
            },
            _ => panic!("doesn't work for enums and unions"),
        }
    } else {
        match input.data {
            syn::Data::Struct(_) => parse_quote! {
                #[derive(
                    serde::Serialize,
                    serde::Deserialize,
                    Clone,
                    Debug,
                    PartialEq,
                )]
                #[serde(deny_unknown_fields)]
                #[derive(derive_from_ext::From)]
                #[from(#ident)]
                #input
            },
            _ => panic!("doesn't work for enums and unions"),
        }
    }
}

pub fn serde_impl(input: DeriveInput) -> DeriveInput {
    match input.data {
        syn::Data::Struct(_) => parse_quote! {
            #[derive(
                serde::Serialize,
                serde::Deserialize,
                Clone,
                Debug,
                PartialEq,
                Eq,
                Default,
            )]
            #[serde(deny_unknown_fields)]
            #input
        },
        syn::Data::Enum(_) => parse_quote! {
            #[derive(
                serde::Serialize,
                serde::Deserialize,
                Clone,
                Debug,
                PartialEq,
            )]
            #[serde(deny_unknown_fields, rename_all = "snake_case")]
            #input
        },
        syn::Data::Union(_) => panic!("unions are not supported"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_structs() {
        let expanded = serde_impl(parse_quote! {
            pub struct InstantiateMsg {
                pub verifier: String,
                pub beneficiary: String,
            }
        });

        let expected = parse_quote! {
            #[derive(
                serde::Serialize,
                serde::Deserialize,
                Clone,
                Debug,
                PartialEq,
                Eq,
                Default,
            )]
            #[serde(deny_unknown_fields)]
            pub struct InstantiateMsg {
                pub verifier: String,
                pub beneficiary: String,
            }
        };

        assert_eq!(expanded, expected);
    }
}
