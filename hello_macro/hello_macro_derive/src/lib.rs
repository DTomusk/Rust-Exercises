extern crate proc_macro;

// crate that allows us to read and manipulate rust code from our code
use proc_macro::TokenStream;
// turns syn data structures back into rust code
use quote::quote;
// parses rust code from a string into a data structure
use syn;

// this function is responsible for parsing the TokenStreams
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // this will generally stay the same for all macros
    // that we can manipulate
    // we use unwrap because this function returns a TokenStream not a result
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    // responsible for transforming the syntax tree
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // returns an Ident struct instance
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // the way this users #name is apparently cool
                // stringify is a rust function that converts an expression into a string literal 
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // consumes intermediate type and returns a TokenStream (as required by the function signature)
    gen.into()
}
