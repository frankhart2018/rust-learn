
fn main() {
    //
    // ..= for matching range of values
    //

    let x = 5;

    match x {
        1..=5 => println!("One through five"),
        _ => println!("Something else"),
    }

    // Ranges are only allowed with numeric values or char values
    // because the compiler checks that the range isn't empty at compile time

    // The only types for which Rust can tell if a range is empty or not
    // are char and numeric values

    let x = 'c';

    match x {
        'a'..='j' => println!("Early ASCII letters"),
        'k'..='z' => println!("Late ASCII letters"),
        _ => println!("Something else"),
    }
}
                