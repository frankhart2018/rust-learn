fn main() {
    // Creating a String
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // Updating a String

    // Appending to a string with push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str takes string slice (borrows) and does not take ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push takes a single character
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with + or format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    // + has a signature for String -> fn add(self, s: &str) -> String, that's why we wrote &s2
    // Although &s2 has a type &String and not &str but the compiler handles it using 'deref coercion'

    // s1's ownership is transferred to s3

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    // For compilcated string concat we can use format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // We cannot index individual character from string in Rust
    // This is because in memory Strings are represented as Vec<u8> and in UTF-8 some characters take more than one byte
    // The length thus returns the length of the vector and indexing will give the index of vector which might not be 
    // a valid UTF-8 character, so Rust stops this

    // Also it is inefficient as in Rust it would have to traverse the vector which will take time

    // So to index we can use string slicing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s);

    // Iterating over string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Iterate over string - bytes method
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
