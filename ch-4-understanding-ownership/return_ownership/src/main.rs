fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    println!("{}", s3); // s2 loses ownership, s3 gets it

    // Multiple return values
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1); // Returned same string just to give back ownership
    println!("The length of '{}' is {}.", s2, len); 
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    // This will give ownership of some_string to the variable that catches this return value
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // This will first recieve ownership from the argument variable and then give it back by returning
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}