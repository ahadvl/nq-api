use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse2, parse_macro_input, Attribute, ItemFn};

/// Parses the args given to the proc macro
/// only parses the string literals
///
/// example
///
/// #[requires("user", "org", "delete")]
///
/// returns: Vec ["user", "org", "delete"]
fn parse_string_attrs<T>(tokens: &mut T) -> Vec<String>
where
    T: Iterator<Item = TokenTree>,
{
    let mut result = vec![];

    while let Some(token) = tokens.next() {
        match token{
            // Skip the tokens like ,
            TokenTree::Punct(_) => continue,

            TokenTree::Literal(lit) => result.push(lit.to_string()),

            tkn => panic!("unexpected token {:?}", tkn),
        }
    }

    result
}

/// requires macro is defines the required permission of the router
///
/// for example we have a delete_account route,
/// and its requires owner permission
///
/// we define it like this #[requires("user", "account", "delete")]
///
/// This macro will get the required permission
/// and triger the casbin permission check function
#[proc_macro_attribute]
pub fn requires(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let mut tokens = attr.into_iter();
    let attrs = parse_string_attrs(&mut tokens);
    println!("{:?}", attrs);

    let out = quote! {
        #input
    };

    out.into()
}
