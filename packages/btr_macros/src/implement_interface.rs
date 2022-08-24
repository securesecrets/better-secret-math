use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, Ident, Type};

const GENERATED_TYPE_SUFFIX: &'static str = "IMPL";
const HAS_INTERFACE_ATTRIBUTE: &'static str = "has_interface";

fn ident_impl(ident: &Ident) -> Ident {
    Ident::new(
        (ident.to_string() + GENERATED_TYPE_SUFFIX).as_ref(),
        Span::call_site(),
    )
}

pub(crate) fn impl_support_interface(ast: DeriveInput) -> TokenStream {
    let name = ast.ident;
    let impl_name = ident_impl(&name);

    // TODO: add support for none, pub, pub(crate) for parameters and the struct
    // TODO: implement From<name> for impl_name

    let generated = match ast.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(FieldsNamed { named, .. }) => {
                // struct parameter names
                let mut names = vec![];
                // struct parameter types
                let mut types = vec![];

                for item in named.iter() {
                    // Check if were defining an interface
                    if item
                        .attrs
                        .iter()
                        .find(|attribute| attribute.path.is_ident(HAS_INTERFACE_ATTRIBUTE))
                        .is_some()
                    {
                        match &item.ty {
                            Type::Path(tp) => {
                                let mut new_tp = tp.clone();
                                let mut path = new_tp.path.segments.first_mut().unwrap();
                                path.ident = ident_impl(&path.ident);

                                types.push(Type::Path(new_tp));
                            }
                            _ => panic!("Attribute must be a struct"),
                        }
                    } else {
                        // TODO: edit convert_to_btr so it works on this
                        types.push(item.ty.clone());
                    }
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
            Fields::Unnamed(_) => panic!("Newtypes not implemented"), //TODO: implement support for unnamed newtypes
            Fields::Unit => panic!("Units not implemented"),
        },
        Data::Enum(_) => panic!("Enum not implemented"), //TODO: implement support for enums
        Data::Union(_) => panic!("Union not implemented"),
    };

    TokenStream::from(generated)
}
