extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Group, Delimiter, Punct, Spacing};


#[proc_macro]
pub fn array(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    // println!("ayy {:#?}", tokens);

    let (func, num) = tokens.split_at(tokens.iter().position(|item| match item {
        TokenTree::Punct(punct) => punct.as_char() == ';',
        _ => false,
    }).unwrap());

    let mut func_vec = func.to_vec();

    let needs_parens = match func_vec.last().unwrap() {
        TokenTree::Group(group) => group.delimiter() != Delimiter::Parenthesis,
        _ => true,
    };

    if needs_parens {
        func_vec.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())));
    }

    let num_repeats: usize = match num.last().unwrap() {
        TokenTree::Literal(lit) => lit.to_string().parse().unwrap(),
        _ => panic!("cant parse number of elements"),
    };

    let mut repeated_funcs = TokenStream::new();
    for i in 0..num_repeats {
        let mut cloned_func = func_vec.clone();

        if i < num_repeats - 1 {
            cloned_func.push(Punct::new(',', Spacing::Alone).into());
        }

        repeated_funcs.extend(cloned_func);
    }

    TokenTree::Group(Group::new(Delimiter::Bracket, repeated_funcs)).into()
}
