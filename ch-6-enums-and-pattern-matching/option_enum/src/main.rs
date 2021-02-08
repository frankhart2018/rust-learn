#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");

    let absent_number: Option<i32> = Option::None;

    println!("{:#?}, {:#?}", some_number, some_string);
    println!("{:#?}", absent_number);
}