
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // This will match anything inside Some()
        // since this y is not the global y (= 10), and is a shadowed variable
        // it matches Some(5) and prints 5
        Some(y) => println!("Matched, y = {:?}", y), 
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {:?}", x, y);
}
                