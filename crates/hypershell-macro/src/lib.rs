use proc_macro::TokenStream;

#[proc_macro]
pub fn hypershell(body: TokenStream) -> TokenStream {
    body
}
