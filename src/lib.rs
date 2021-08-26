extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree, Group, Delimiter, Punct, Spacing};

#[proc_macro]
pub fn array(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    // finds the semi-colon seperator
    let (func, num) = tokens.split_at(tokens.iter().position(|item| match item {
        TokenTree::Punct(punct) => punct.as_char() == ';',
        _ => false,
    }).unwrap());

    let mut func_vec = func.to_vec();

    // Checks if we need to add parenthesis to the function call
    let needs_parens = match func_vec.last().unwrap() {
        TokenTree::Group(group) => group.delimiter() != Delimiter::Parenthesis,
        _ => true,
    };

    // add if needed
    if needs_parens {
        func_vec.push(TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())));
    }

    // parse the array size
    let num_repeats: usize = match num.last().unwrap() {
        TokenTree::Literal(lit) => lit.to_string().parse().unwrap(),
        _ => panic!("cant parse number of elements"),
    };

    // build the array by repeating the function call as many times as needed
    let mut repeated_funcs = TokenStream::new();
    for i in 0..num_repeats {
        let mut cloned_func = func_vec.clone();

        if i < num_repeats - 1 {
            cloned_func.push(Punct::new(',', Spacing::Alone).into());
        }

        repeated_funcs.extend(cloned_func);
    }
    
    // wrap it all in brackets and return
    TokenTree::Group(Group::new(Delimiter::Bracket, repeated_funcs)).into()
}
