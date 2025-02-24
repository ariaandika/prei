use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, DeriveInput, Token, Type};

mod macros;

#[proc_macro_derive(Ts)]
pub fn ts(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match macros::parse_derive(input.into()) {
        Ok(ok) => ok.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    match syn::parse::<Types>(input) {
        Ok(Types(ty)) => {
            let ty = ty.iter();
            quote! {{
                let mut output = String::new();
                #(output += &<#ty as ::prei::Interface>::generate();)*
                output
            }}.into()
        },
        Err(err) => err.into_compile_error().into(),
    }
}

struct Types(Punctuated<Type,Token![,]>);

impl syn::parse::Parse for Types {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(Punctuated::parse_terminated)?))
    }
}

