use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, *};


pub fn parse_derive(input: DeriveInput) -> Result<TokenStream> {
    return match &input.data {
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
                        <#ty as ::prei::Type>::gen_type_to(buffer);
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
                <#ty as ::prei::Type>::gen_type_to(buffer);
                buffer.push_str(";\n");
            }
        },
        Fields::Unnamed(fields) => {
            let fields = fields.unnamed.iter().map(|e|{
                let ty = &e.ty;
                quote! {
                    <#ty as ::prei::Type>::gen_type_to(buffer);
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

    Ok(quote! {
        #[cfg(debug_assertions)]
        impl ::prei::Type for #ident {
            fn gen_type_to(buffer: &mut String) {
                buffer.push_str(#ident_str);
            }
        }
        #[cfg(debug_assertions)]
        impl ::prei::Interface for #ident {
            fn gen_interface_to(buffer: &mut String) {
                buffer.push_str("export type ");
                buffer.push_str(#ident_str);
                buffer.push_str(" = ");
                #generated;
            }
        }
    })
}

