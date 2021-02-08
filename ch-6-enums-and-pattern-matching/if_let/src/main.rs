fn main() {
    // Match
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if-let
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("Hello")
    }
}
