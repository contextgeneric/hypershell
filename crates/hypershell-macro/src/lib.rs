use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

mod expand;

#[proc_macro]
pub fn hypershell(body: TokenStream) -> TokenStream {
    let stream =
        expand::process_extended_token_tree(&mut TokenStream2::from(body).into_iter(), false);
    expand::expand_and_pipe(expand::ExtendedTokenStream { stream }).into()
}
