fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Ownership stays with s1, this is called parameter borrowing
    println!("The length of '{}' is {}.", s1, len);

    // change(&s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(some_string: &String) {
    // some_string.push_str(", world"); // This will throw an error as we cannot change a borrowed value
// }