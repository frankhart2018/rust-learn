fn main() {
    let str1 = "somnambulance".to_string();
    let str2 = str1; // This moves the string from str1 to str2

    println!("{}", str2);
    
    let num1: i32 = 36;
    let num2 = num1; // This copies the value to num2, since this copy operation is cheap

    println!("{}, {}", num1, num2);

    // These types are called Copy types in Rust
    // The source of the assignment remains initialized and unsable, with the same value it had before
    // Passing Copy types to functions and constructors behaves similarly
    
    // The standard Copy types include all the machine integer and floating-point numeric types,
    // the char and bool types

    // As a rule of thumb, any type that needs to do something special when a value is dropped
    // cannot be a Copy
    // A Vec needs to free its elements, a File needs to close its file handle, 
    // a MutexGuard needs to unlock its mutex

    // By default struct and enum types (user-defined types) are not Copy

    struct Label {
        number: u32
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label {
        number: 3
    };
    print(l);
    // println!("My label number is: {}", l.number); // This will produce error as number has been moved to print() function

    // If all the fields of struct are Copy themselves then we can make the type of struct Copy as well
    #[derive(Copy, Clone)]
    #[allow(unused)]
    struct NewLabel {
        number: u32,
    }

    // However if we try to change the type of struct to Copy and the fields are not of Copy type then it will throw error
    // #[derive(Copy, Clone)]
    // struct StringLabel {
    //     name: String
    // }
}
