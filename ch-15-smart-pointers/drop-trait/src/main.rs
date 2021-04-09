fn main() {
    // Drop trait lets us customize what happens when a value 
    // is about to go out of scope

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c); // This clears C before it goes out of scope
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    // If we want to clean memory before it goes out of scope
    // we can't directly call drop() method, because Rust will still
    // call drop() method at the end which will result in a double free error
}
