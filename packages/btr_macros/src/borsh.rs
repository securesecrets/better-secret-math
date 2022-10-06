use proc_macro2::Ident;
use syn::{parse_quote, DeriveInput};

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
                #[btr_macros::borsh_serde]
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
                borsh::BorshSerialize,
                borsh::BorshDeserialize,
                Clone,
                Debug,
                PartialEq,
            )]
            #[serde(deny_unknown_fields)]
            #input
        },
        syn::Data::Enum(_) => parse_quote! {
            #[derive(
                serde::Serialize,
                serde::Deserialize,
                borsh::BorshSerialize,
                borsh::BorshDeserialize,
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
                borsh::BorshSerialize,
                borsh::BorshDeserialize,
                Clone,
                Debug,
                PartialEq,
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
