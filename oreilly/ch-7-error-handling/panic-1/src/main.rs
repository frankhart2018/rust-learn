fn main() {
    // A program panics when it encounters something so messed up that there must be a bug
    // in the program itself, some cases are:-

    // 1. Out of bounds array access
    // 2. Integer division by zero
    // 3. Calling .unwrap() on an Option that happens to be None
    // 4. Assertion failure

    // panic! macro can be used to trigger a panic

    // There are two options when something panics - either unwind the stack or abort the process

    // UNWINDING

    fn _pirate_share(total: u64, crew_size: usize) -> u64 {
        let half = total / 2;
        half / crew_size as u64
    }

    // This function will work normally unless crew_size is set to 0
    // In which case it will panic and throw error message - 'attempt to divide by zero'
    // Then the stack is unwound, dropping all local variables from the function then moves to called
    // This is done until the stack is empty
    // Finally the thread exits, if the panicking thread was the main thread then the whole process exits

    // ABORTING

    // There are two circumstances in which Rust does not try to unwind the stack:-

    // 1. If a .drop() method triggers a second panic while Rust is still trying to clean up
    // the first panic, this is considered fatal, Rust stops unwinding and aborts the process

    // 2. The panic behaviour is customizable if we compile with -C panic=abort
    // it will never try to unwind
}
