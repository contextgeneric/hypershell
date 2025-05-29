use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{ToTokens, quote};

pub fn expand(tokens: impl Iterator<Item = TokenTree>) -> Vec<TokenStream> {
    let mut tokens = tokens.peekable();

    let mut out = TokenStream::new();

    loop {
        match tokens.next() {
            Some(token) => match token {
                TokenTree::Literal(literal) => out.extend(quote! {
                    symbol!( #literal )
                }),
                TokenTree::Group(group) => {
                    let in_expanded = expand_and_pipe(group.stream().into_iter());
                    let new_group = Group::new(group.delimiter(), in_expanded);
                    out.extend(new_group.to_token_stream());
                }
                TokenTree::Ident(ident) => {
                    if let Some(TokenTree::Group(group)) = tokens.peek() {
                        if group.delimiter() == Delimiter::Bracket {
                            let in_expanded = expand_and_pipe(group.stream().into_iter());

                            out.extend(quote! {
                                #ident < Product![ #in_expanded ] >
                            });

                            tokens.next();
                        }
                    }
                }
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '|' {
                        let mut expanded = expand(tokens);
                        expanded.insert(0, out);
                        return expanded;
                    }
                }
            },
            None => return vec![out],
        }
    }
}

pub fn expand_and_pipe(tokens: impl Iterator<Item = TokenTree>) -> TokenStream {
    token_streams_to_pipe(expand(tokens))
}

pub fn token_streams_to_pipe(mut token_streams: Vec<TokenStream>) -> TokenStream {
    let count = token_streams.len();

    if count == 0 {
        TokenStream::new()
    } else if count == 1 {
        token_streams.pop().unwrap()
    } else {
        let mut out = TokenStream::new();

        for token_stream in token_streams.into_iter() {
            out.extend(token_stream);
            out.extend(quote! { , });
        }

        quote! { Pipe< #out > }
    }
}
