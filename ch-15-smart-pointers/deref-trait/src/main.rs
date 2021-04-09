use std::ops::Deref;

fn main() {
    // Regular dereferencing
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Using Box<T> like reference
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // The only difference here is that y is an instance of Box
    // pointing to a copied value of x rather than a reference pointing to the value of x

    // 
    // Defining our own smart pointer
    // 

    // Box<T> is defined using tuple struct
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T; // Defines associated type for the Deref trait to use

        fn deref(&self) -> &T {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Behind the scenes for *y Rust executes *(y.deref())

    //
    // Deref coercion
    //

    // Deref coercion is a convenience that Rust performs on arguments to functions and methods
    // Deref coercion works only on types that implement the Deref trait
    // Deref coercion converts such a type into a reference to another type
    // For example it can convert &String to &str because String implements Deref trait
    // such that it returns str.

    fn hello(name: &str) {
        println!("Hello, {}", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // Converts &MyBox<String> to &String by calling deref
    // Rust then calls the implementation of String to convert &String to &str
    // which calls deref method again

    // If there was no deref coercion we would have to write
    // hello(&(*m)[..]);

    // Here (*m) dereferences the MyBox<String> into a String
    // Then the & and [..] take a string slice of the String
    // that is equal to the whole string to match the signature of hello

    // DerefMut trait can be used to override * operator on mutable references

    // Rust does deref coercion in the following cases:-

    // 1. From &T to &U when T: Deref<Target=U>

    // 2. From &mut T to &mut U when T: DerefMut<Target=U>

    // 3. From &mut T to &U when T: Deref<Target=U>

    // The third one means that Rust will coerce mutable reference to an immutable one
    // This is due to the borrowing rules, there can be multiple immutable references
    // but only one mutable reference
}
