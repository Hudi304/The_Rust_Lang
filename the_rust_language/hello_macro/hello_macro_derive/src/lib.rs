use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // proc_macro comes with rust and is the compiler's API
    // that allows you to manipulate rust code

    // the syn crate parses the code from a string
    // into some other data structure

    // the quote create turns syn data strictures back into Rust code
    // (these crates make it simpler to manipulate code)
    // writing a rust parses is not a simple task

    // this macro will be called on [derive(HelloMacro)]

    // Build the trait implementation
    return impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    return gen.into();
}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
