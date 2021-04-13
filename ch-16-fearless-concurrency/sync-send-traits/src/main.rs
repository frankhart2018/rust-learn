fn main() {
    // 
    // Allowing transference of ownership between threads with send
    //

    // The Send marker trait indicates that the ownership of the type
    // implementing Send can be transferred between threads

    // Almost every Rust type is Send, with some exceptions like Rc<T>
    // This does not implement Send because if we clone Rc<T> and tried to 
    // transfer ownership of the clone to another thread, both can update the
    // reference count at the same time

    //
    // Allowing access from multiple threads with Sync
    //

    // The Sync marker trait indicates that it is safe for the type
    // implementing Sync to be referenced from multiple threads
    // T is Sync if &T is Send, meaning the reference can be sent 
    // to another thread safely
    // Primitive types are Sync
    
    // Rc<T> is not Sync too for same reasons as it is not Send
    // The RefCell<T> and the family of Cell<T> types are not Sync
    // This is because the implementation of borrow checking that RefCell<T> does
    // at runtime is not thread safe

    // The smart pointer Mutex<T> is Sync and can be used to share access
    // with multiple threads 

    //
    // Implementing Send and Sync manually is unsafe
    //

    // Because types that are made up of Send and Sync traits are
    // automatically Send and Sync, we don't have to implement these traits manually
    // As marker traits they don't have any methods to implement
    // They are just useful for enforcing invariants related to concurrency
}
