use std::rc::Rc;

fn main() {
    // Rc and Arc are used for reference counted pointers in Rust
    // The difference between them is that an Arc is safe to share between threads directly
    // the name Arc is short for atomic reference count 
    // whereas a plain Rc uses faster non-thread safe code to update its reference count

    // Rust can infer all these types; written out only for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    // For any type T, an Rc<T> value is a pointer to a heap allocated T that has had a reference count affixed to it
    // Cloning an Rc<T> value does not copy the T, instead it simply created another pointer to it
    // and increments the reference count

    // The String is droppped only when the last Rc is dropped

    // All String methods work with Rc<String>
    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // A value owned by an Rc pointer is immutable
    // s.push_str(" noodles"); // This will throw error, as s is Rc type

    // Rust's memory and thread-safety guarantees depend on ensuring that no value is ever
    // simultaneously shared and mutable, Rust assumes the referent of an Rc pointer might in
    // general be shared, so it must not be mutable

    // One problem is when a Rc value points to another Rc value
    // Then the values will never be freed and it is possible to leak values in Rust this way
}
