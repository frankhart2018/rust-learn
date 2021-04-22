
fn main() {
    // There are three kind of procedural macros:-

    // 1. Custom #[derive] macros that specify code added with the derive
    // attribute used on structs and enums

    // 2. Attribute-like macros that define custom attributes usable on any item

    // 3. Function-like macros that look like functionc calls but operate on the 
    // tokens specified as their argument

    //
    // Difference between macros and functions
    //

    // Fundamentally the macros are a way of writing other code which is known as 
    // metaprogramming

    // A function signature must declare the number and type of parameters the functions has
    // but macros can take a variable number of parameters

    // Macros are expanded before the compiler interprets the meaning of code
    // so a macro can implement a trait on a given type, a function can't do this
    // because it gets called in runtime and a trait needs to be implemented at compile time

    // Macros are tough to read, write and maintain than function defintions

    // Macros need to be brought into scope before we call them in a file
    // Functions can be defined anywhere and called anywhere

    //
    // Declarative macros with macro_rules! for general metaprogramming
    // 

    // These allow us to write something similar to Rust match expression
    // Like match expressions compare value and execute code accordingly
    // macros also compare a value to patterns but value in this case is literal Rust code

    #[macro_export]
    macro_rules! vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    // #[macro_suport] indicates that this macro should be made available whenever
    // the crate in which the macro is defined is brought into scope
    // Macro can't be brought into scope without this

    // Description of the pattern
    // First a set of parantheses surrounds the whole pattern
    // A $ is next followed by a set of parantheses that captures values
    // that match the pattern within the parantheses for use in the replacement code
    // $x: expr matches any Rust expression and gives the expression the name $x
    // , after this indicates literal comma separator can optionally appear after the code
    // The * specifies that the pattern matches zero or more of whatever precedes the *
    // The code inside is generated for each pattern that matches $()*
    // The $x is replaced with each expression matched

    // The following code is generated for vec![1, 2, 3];

    // {
    //     let mut temp_vec = Vec::new();
    //     temp_vec.push(1);
    //     temp_vec.push(2);
    //     temp_vec.push(3);
    //     temp_vec
    // }

    //
    // Procedural macros for generating code for attributes
    //

    // They accept some code as an input, operate on the code,
    // and produce some code as an output rather than matching against patterns
    // and replacing code with other code as declarative macros do

    // There are three kinds of procedural macros - custom derive, attribute like, function like

    // When creating procedural macros, the definitions must reside in their own crate
    // with a special crate type

    // use proc_macro;

    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {}

    // TokenStream type is defined by the proc_macro crate, this represents a 
    // sequence of tokens

    // The function also has an attribute attached to it that specifies which kind 
    // of procedural macro we're creating

    // We can have multiple kinds of procedural macros in the same crate
}
                