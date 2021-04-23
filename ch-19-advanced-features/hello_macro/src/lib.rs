pub trait HelloMacro {
    fn hello_macro();
}

// Using just the trait users can implement it on their types
// We can't provide hello_macro() a default implementation 
// because Rust doesn't have reflection capabilities, so it can't look
// up the type's name at runtime, so we need a macro to generate code at compile time

// Procedural macros need their own crate, so created hello_macro_derive