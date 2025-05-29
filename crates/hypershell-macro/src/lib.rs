use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod expand;

#[proc_macro]
pub fn hypershell(body: TokenStream) -> TokenStream {
    expand::expand_and_pipe(TokenStream2::from(body)).into()
}
