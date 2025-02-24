use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, *};
use super::TsType;

pub fn parse_derive(input: DeriveInput) -> Result<TokenStream> {
    match input.data {
        Data::Struct(_) => parse_struct(input),
        Data::Enum(_) => parse_enum(input),
        Data::Union(_) => Err(Error::new(input.span(), "Union not supported"))
    }
}

fn parse_struct(DeriveInput { ident, data, generics, .. }: DeriveInput) -> Result<TokenStream> {
    let Data::Struct(data) = data else { unreachable!("matches") };
    let (g1,g2,g3) = generics.split_for_impl();
    let ident_str = ident.to_string();
    let head = format!("export type {ident_str} = ");

    let generated = match &data.fields {
        Fields::Named(fields) => {
            let fields = fields
                .named
                .iter()
                .map(|e| e.ident.as_ref().expect("named").to_string())
                .zip(&data.fields)
                .map(|(id, e)| {
                    let ty = &e.ty;
                    let head = format!("  {id}: ");
                    quote! {
                        buffer.push_str(#head);
                        <#ty as #TsType>::gen_id_to(buffer);
                        buffer.push_str(",\n");
                    }
                });

            let head = format!("{head}{{\n");
            quote! {
                buffer.push_str(#head);
                #(#fields)*
                buffer.push_str("};\n");
            }
        },
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
            let ty = fields.unnamed.first().expect("one");
            quote! {
                buffer.push_str(#head);
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
            let head = format!("{head}[");
            quote! {
                buffer.push_str(#head);
                #(#fields)*;
                buffer.push_str("];\n");
            }
        }
        Fields::Unit => {
            let head = format!("{head}null;");
            quote! {
                buffer.push_str(#head);
            }
        },
    };

    Ok(quote! {
        #[cfg(debug_assertions)]
        impl #g1 #TsType for #ident #g2 #g3 {
            fn gen_id_to(buffer: &mut String) {
                buffer.push_str(#ident_str);
            }

            fn gen_type_to(buffer: &mut String) {
                #generated;
            }
        }
    })
}

fn parse_enum(DeriveInput { ident, data, generics, .. }: DeriveInput) -> Result<TokenStream> {
    let Data::Enum(data) = data else { unreachable!("matched") };
    let (g1,g2,g3) = generics.split_for_impl();
    let ident_str = ident.to_string();
    let head = format!("export type {ident_str} = ");

    let generated = data.variants.into_iter().map(|variant| {
        match &variant.fields {
            Fields::Named(fields) => {
                let fields = fields
                    .named
                    .iter()
                    .map(|e| e.ident.as_ref().expect("named").to_string())
                    .zip(&variant.fields)
                    .map(|(id, e)| {
                        let ty = &e.ty;
                        let head = format!("    {id}: ");
                        quote! {
                            buffer.push_str(#head);
                            <#ty as #TsType>::gen_id_to(buffer);
                            buffer.push_str(",\n");
                        }
                    });

                let head = format!("  | {{\n    tag: {:?},\n    value: {{",variant.ident);
                quote! {
                    buffer.push_str(#head);
                    #(#fields)*
                    buffer.push_str("    }\n  }\n");
                }
            },
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let ty = fields.unnamed.first().expect("one");
                let head = format!("  | {{\n    tag: {:?},\n    value: ",variant.ident);
                quote! {
                    buffer.push_str(#head);
                    <#ty as #TsType>::gen_id_to(buffer);
                    buffer.push_str("\n  }\n");
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
                let head = format!("  | {{\n    tag: {:?},\n    value: [",variant.ident);
                quote! {
                    buffer.push_str(#head);
                    #(#fields)*;
                    buffer.push_str("]\n  }\n");
                }
            }
            Fields::Unit => {
                let head = format!("  | {{\n    tag: {:?},\n    value: null\n  }}\n",variant.ident);
                quote! {
                    buffer.push_str(#head);
                }
            },
        }
    });

    Ok(quote! {
        #[cfg(debug_assertions)]
        impl #g1 #TsType for #ident #g2 #g3 {
            fn gen_id_to(buffer: &mut String) {
                buffer.push_str(#ident_str);
            }

            fn gen_type_to(buffer: &mut String) {
                buffer.push_str(#head);
                #(#generated)*
            }
        }
    })
}

