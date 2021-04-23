extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // Get the name of type
    let name = &ast.ident; 
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // #name will be replaced by variable name
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // Convert it back into TokenStream
    gen.into()
}

// hello_macro_derive() is responsble for parsing the TokenStream

// impl_hello_macro() is responsible for transforming the syntax tree
// this makes writing a procedural macro more convenient

// proc_crate is compiler's API that allows us to read 
// and manipulate Rust code from our code

// The syn crate parses Rust code from a string into a data structure
// that we can perform operations on

// The quote crate turns syn data structures back into Rust code

// The hello_macro_derive function will be called when a user of our library
// specified #[derive(HelloMacro)] on a type, this is possible because we've
// annotated the hello_macro_derive function here with proc_macro_derive
// and specified the name HelloMacro which matches our trait's name
