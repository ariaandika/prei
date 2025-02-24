use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, *};
use super::TsType;

pub fn parse_derive(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(data) => parse_struct(data.clone(), input),
        syn::Data::Enum(_) => todo!(),
        syn::Data::Union(_) => Err(Error::new(input.span(), "Union not supported"))
    }
}

fn parse_struct(data: DataStruct, input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let ident_str = input.ident.to_string();

    let generated = match &data.fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .map(|e| e.ident.as_ref().expect("named").to_string())
                .zip(&data.fields)
                .map(|(id, e)| {
                    let ty = &e.ty;
                    quote! {
                        buffer.push_str("  ");
                        buffer.push_str(#id);
                        buffer.push_str(": ");
                        <#ty as #TsType>::gen_id_to(buffer);
                        buffer.push_str(",\n");
                    }
                });

            quote! {
                buffer.push_str("{\n");
                #(#fields)*
                buffer.push_str("};\n");
            }
        },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = fields.unnamed.first().expect("one");
            quote! {
                <#ty as #TsType>::gen_id_to(buffer);
                buffer.push_str(";\n");
            }
        },
        Fields::Unnamed(fields) => {
            let fields = fields.unnamed.iter().map(|e|{
                let ty = &e.ty;
                quote! {
                    <#ty as #TsType>::gen_id_to(buffer);
                    buffer.push_str(",");
                }
            });
            quote! {
                buffer.push('[');
                #(#fields)*;
                buffer.push_str("];\n");
            }
        }
        Fields::Unit => quote! { null; },
    };

    let head = format!("export type {ident_str} = ");

    Ok(quote! {
        #[cfg(debug_assertions)]
        impl #TsType for #ident {
            fn gen_id_to(buffer: &mut String) {
                buffer.push_str(#ident_str);
            }

            fn gen_type_to(buffer: &mut String) {
                buffer.push_str(#head);
                #generated;
            }
        }
    })
}

