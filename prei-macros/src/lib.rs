use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, DeriveInput, Token, Type};

mod macros;

#[proc_macro_derive(Ts)]
pub fn ts(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match macros::parse_derive(input) {
        Ok(ok) => ok.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn generate_type(input: TokenStream) -> TokenStream {
    let ts_type = TsType.into_token_stream();
    match syn::parse::<Types>(input) {
        Ok(Types(ty)) => {
            let ty = ty.iter();
            quote! {{
                let mut output = String::new();
                #(<#ty as #ts_type>::gen_type_to(&mut output);)*
                output
            }}.into()
        },
        Err(err) => err.into_compile_error().into(),
    }
}

struct TsType;

impl ToTokens for TsType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! { ::prei::TsType });
    }
}

struct Types(Punctuated<Type,Token![,]>);

impl syn::parse::Parse for Types {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.call(Punctuated::parse_terminated)?))
    }
}

