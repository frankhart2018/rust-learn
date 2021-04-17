
fn main() {
    // The at operator (@) lets us create a variable that holds
    // a value at the same time we're testing that value to see
    // whether it matches a pattern

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    // @ pattern lets us test a value and save it 
    // in a variable within one pattern
}
                