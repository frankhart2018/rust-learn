fn main() {
    let s = String::from("hello");
    
    takes_ownership(s); // Same as variable ownership, s will be moved to some_string

    // This will throw an error
    // println!("{}", s);

    let x = 5;

    makes_copy(x); // Same as variable ownership, x will be copied (not moved) to some_integer
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}