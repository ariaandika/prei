use proc_macro::TokenStream;
use syn::DeriveInput;

mod macros;

#[proc_macro_derive(Ts)]
pub fn ts(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    match macros::parse_derive(input.into()) {
        Ok(ok) => ok.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

