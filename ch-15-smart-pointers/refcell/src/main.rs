fn main() {
    // Interior mutability is a design pattern in Rust
    // that allows us to mutate data even when there are immutable
    // references to that data

    // To mutate data this pattern uses unsafe code inside a data structure
    // to bend Rust's usual rules that govern mutability and borrowing

    // We can use types that use the interior mutability pattern when
    // we can ensure that the borrowing rules will be followed at runtime
    // even though the compiler can't guarantee that

    // The unsagfe code involved is then wrapped in a safe API and the outer
    // type is still immutable

    // RefCell<T> represents single ownership over the data, like Box<T>
    // The difference is that the borrowing rules in Box<T> are enforced 
    // during compile time, but in case of RefCell<T> it is done at runtime
    // With Box<T>, if we break these rules the compiler will throw error,
    // but with RefCell<T> if we break these rules then the program will panic and exit

    // RefCell<T> is also only for use in single-threaded scenarios

    // 
    // Reasons to choose from Box<T>, Rc<T>, or RefCell<T>
    //

    // Rc<T> enables multiple owners of the same data, Box<T> and RefCell<T>
    // have single owners

    // Box<T> allows immutable or mutable borrows checked at compile time
    // Rc<T> allows only immutable borrows checked at compile time
    // RefCell<T> allows immutable or mutable borrows checked at runtime

    // Because RefCell<T> allows mutable borrows checked at runtime, we can
    // mutate the value inside RefCell<T> even when the RefCell<T> is immutable

    //
    // Working of RefCell<T>
    //

    // borrow() method returns the smart pointer type Ref<T>
    // and borrow_mut() method returns the smart pointer type RefMut<T>
    // Both of these implement Deref trait, so they can be treated as regular references

    // RefCell<T> keeps track of how many Ref<T> and RefMut<T> are currently active
    // Just like compile time borrows RefCell<T> allows us to have many immutable 
    // borrows or one mutable borrow at any point in time

    // Using RefCell<T> is a bit computationally expensive, as it has to track
    // borrows at runtime
}
