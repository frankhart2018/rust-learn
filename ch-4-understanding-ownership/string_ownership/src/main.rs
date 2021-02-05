fn main() {
    let s = String::from("hello");
    println!("s: {}", s);

    let mut s = String::from("hello"); 
    s.push_str(", world");
    println!("s: {}", s);

    // The memory is freed when the variable that owns it goes out of scope
    // Rust calls a method called 'drop' which does this job

    let s1 = String::from("hello");
    let s2 = s1; // Here only the pointer, length, capacity from stack get copied, the data in heap doesn't
    println!("s2: {}", s2);

    // When s1 and s2 go out of scope, both will try to drop the value which can lead to double free error
    // To solve this rust makes s1 invalid so that only s2 will drop the value

    // This will cause error
    // println!("s1: {}", s1);

    // Since s1 was invalidated so s2 = s1 is not shallow copy but rather called 'move' operation

    // Deep copy (not efficient)
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    // For simple types like integer everything is saved in the stack so there isn't any difference between
    // shallow and deep copy
}