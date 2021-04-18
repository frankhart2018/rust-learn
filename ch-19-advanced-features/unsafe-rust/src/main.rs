use std::slice;

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // Five actions (or five superpowers) of unsafe rust:-

    // 1. Dereference a raw pointer

    // 2. Call an unsafe function or method

    // 3. Access or modify a mutable static method

    // 4. Implement a unsafe trait

    // 5. Access fields of union S

    // There is still some level of safety inside unsafe block

    //
    // Dereferencing a raw pointer
    //

    // The first type in unsafe Rust is raw pointers
    // It can be immutable (*const T) or mutable (*mut T) here * is not dereference operator
    // Immutable here means the pointer can't be directly assigned to after being dereferenced

    // Differences between raw pointers and references or smart pointers:-

    // 1. Are allowed to ignore the borrowing rules by having both 
    // immutable and mutable pointers or multiple pointers to the same location

    // 2. Aren't guaranteed to point to valid memory

    // 3. Are allowed to be null

    // 4. Don't implement any automatic cleanup

    // Creating immutable and mutable raw pointers from references

    let mut num = 5;

    let _r1 = &num as *const i32; // immutable raw pointer
    let _r2 = &mut num as *mut i32; // mutable raw pointer

    // We can create raw pointers in safe Rust too, we just can't dereference them here

    // Raw pointer to arbitrary memory location
    let address = 0x012345usize;
    let _r = address as *const i32;

    // Dereference raw pointers in unsafe block

    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Note:- Creating a pointer does no harm, only when we try to access the value
    //        that is points at that we might end up dealing with an invalid value

    //
    // Calling an unsafe function or method
    //

    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

    // Bodies of unsafe functions are effectively unsafe blocks, so to perform
    // unsafe operations within an unsafe function we don't need to add another
    // unsafe block

    // Creating a safe abstraction over unsafe code

    let mut v = vec![1, 2, 3 , 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // To implement it we can't just use safe Rust because it returns a tuple with 
    // two mutable references which is not possible in safe Rust

    #[allow(unused)]
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // Get a raw pointer to slice, returns *mut i32

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let address = 0x012345usize;
    let r = address as *mut i32;

    let _slice: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    //
    // Use extern functions to call external code
    //

    // extern creates FFI (Foreign Function Interface)
    // FFI is a way for a programming language to define funcitons 
    // and enable a different (foreign) programming language to call those functions

    // The reason unsafe block is required here is that other languages
    // do not enforce the same rules as Rust thus using safe Rust will generate errors

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Calling Rust functions from other languages

    // We can add the extern keyword followed by ABI (Application Binary Interface) name
    // We also need to specify #[no_mangle] to avoid name mangling

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    //
    // Accessing or modifying a mutable static variable
    //

    // In Rust global variables are called static variables

    // The convention to use for naming these variables is screaming snake case
    // i.e. upper case and words separated by _ (underscore)

    // Constants and immutable static variables are almost similar
    // except that values in a static variable have a fixed address in memory
    // Another difference is that static variables can be mutable, though
    // modifying mutable static variables is unsafe

    // Variable and funcitons above (added before main), just calling here

    unsafe {
        println!("COUNTER: {}", COUNTER);
        add_to_count(10);
        println!("COUNTER: {}", COUNTER);
    }

    //
    // Implementing an unsafe trait
    //

    // A trait is unsafe when at least one of its methods has some
    // invariant that the compiler can't verify

    // We can declare that a trait is unsafe by adding the unsafe keyword
    // before trait and marking the implementation of the trait as unsafe too

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }

    //
    // Accessing fields of a union
    //

    // Unions are primarily used to interface with unions in C code
    // Accessing union fields is unsafe because Rust can't guarantee the type
    // of the data currently being stored in the union instance
}
                