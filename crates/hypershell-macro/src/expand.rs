use core::iter::Peekable;

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, TokenStream, TokenTree};
use quote::{ToTokens, quote};

pub fn expand_and_pipe(tokens: TokenStream) -> TokenStream {
    token_streams_to_pipe(expand(tokens.into_iter().peekable()))
}

pub fn expand(mut tokens: Peekable<impl Iterator<Item = TokenTree>>) -> Vec<TokenStream> {
    let mut out = TokenStream::new();

    loop {
        match tokens.next() {
            Some(token) => match token {
                TokenTree::Literal(literal) => out.extend(quote! {
                    symbol!( #literal )
                }),
                TokenTree::Group(group) => {
                    let in_expanded = expand_and_pipe(group.stream());
                    let new_group = Group::new(group.delimiter(), in_expanded);
                    out.extend(new_group.to_token_stream());
                }
                TokenTree::Ident(ident) => {
                    out.extend(quote! { #ident });

                    if let Some(TokenTree::Group(group)) = tokens.peek() {
                        if group.delimiter() == Delimiter::Bracket {
                            let in_expanded = expand_and_pipe(group.stream());

                            out.extend(quote! {
                                < Product![ #in_expanded ] >
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
                    } else {
                        out.extend(punct.to_token_stream())
                    }
                }
            },
            None => return vec![out],
        }
    }
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

        quote! { Pipe< Product![ #out ] > }
    }
}

pub enum ExtendedTokenTree {
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
    Group(ExtendedDelimiter, Vec<ExtendedTokenTree>),
}

pub enum ExtendedDelimiter {
    Base(Delimiter),
    AngleBracket,
}

pub fn process_extended_token_tree(
    tokens: &mut impl Iterator<Item = TokenTree>,
    is_inner: bool,
) -> Vec<ExtendedTokenTree> {
    let mut out = Vec::new();

    loop {
        match tokens.next() {
            Some(token) => match token {
                TokenTree::Punct(punct) => {
                    if punct.as_char() == '<' {
                        let inner = process_extended_token_tree(tokens, true);
                        out.push(ExtendedTokenTree::Group(
                            ExtendedDelimiter::AngleBracket,
                            inner,
                        ));
                    } else if punct.as_char() == '>' {
                        return out;
                    } else {
                        out.push(ExtendedTokenTree::Punct(punct));
                    }
                }
                TokenTree::Group(group) => {
                    let inner = process_extended_token_tree(&mut group.stream().into_iter(), false);
                    out.push(ExtendedTokenTree::Group(
                        ExtendedDelimiter::Base(group.delimiter()),
                        inner,
                    ));
                }
                TokenTree::Ident(ident) => {
                    out.push(ExtendedTokenTree::Ident(ident));
                }
                TokenTree::Literal(literal) => {
                    out.push(ExtendedTokenTree::Literal(literal));
                }
            },
            None => {
                if !is_inner {
                    panic!("mismatch > at the end of token stream")
                }

                return out;
            }
        }
    }
}
