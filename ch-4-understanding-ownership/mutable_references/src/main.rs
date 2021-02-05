fn main() {
    let mut s = String::from("hello");

    change(&mut s); // The downside of mutable references is that we can have only one of these in a particular scope
    println!("{}", s);

    // This is to prevent data races which occurs when these three behaviours occur:-
    // - Two or more pointers access the same data at the same time
    // - At least one of the pointers is being used to write to the data
    // - There's no mechanism being used to synchronize access to the data

    // Error is also thrown when mutable and immutable references are used in same scope
    // Multiple immutable references in same scope is fine

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // Scope of r1 and r2 ends so now we can define a mutable reference

    let r3 = &mut s;
    println!("{}", r3);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}