fn main() {
    // POINTERS

    // Type 1 - References

    // Borrows a reference to x

    // Like a C pointer a reference does not automatically free any resources
    // when it goes out of scope

    // Unlike C pointers, Rust references are never null, there is 
    // no way to produce a null reference in Rust, these references are immutable by default
    // &T -> Immutable reference
    // &mut T -> Mutable reference

    // Another difference is that rust tracks ownership and lifetime of values
    // so mistakes like dangling pointers, dangling frees and pointer invalidation are ruled out

    // Type 2 - Boxes

    // Simplest way to allocate a value in heap 
    let t = (12, "eggs"); // Type is (i32, &str) -> &str = Slice of string
    // Prefixing with _ allows variable to be left unused
    let _b = Box::new(t); // Type is Box<(i32, &str)>, the memory will be allocated on the heap
    // When b goes out of scope the memory is freed immediately unless the ownership has been transferred
    // by returning it

    // Type 3 - Raw pointers
    // Raw pointers are like pointers in C++, using a raw pointer is unsafe
    // as rust does not make any effort to track what it points to

    // Dereferencing of raw pointers can only be done inside an unsafe block
}
