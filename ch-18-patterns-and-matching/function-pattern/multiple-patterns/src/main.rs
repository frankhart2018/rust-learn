
fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("One or two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }
}
                