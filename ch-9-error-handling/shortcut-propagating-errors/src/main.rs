use std::fs::File;
use std::io;
use std::io::Read;
use std::error::Error;

// Box<dyn Error> means any type of error
fn main() -> Result <(), Box<dyn Error>> {
    fn read_username_from_file() -> Result <String, io::Error> {
        // ? is for propagating error, if Result<T, E> is Ok then it returns the value
        // If it is Err then it returns from the function 
        // ? also converts the error type to io::Error in this function's case if it is not of that type
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)

        // Chaining ?
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)

        // ? can only be used in a function that returns Result<T, E> or Option 
        // or any other type that implements std::ops::Try

        // To make it even more short
        fs::read_to_string("hello.txt")
    }

    Ok(())
}
