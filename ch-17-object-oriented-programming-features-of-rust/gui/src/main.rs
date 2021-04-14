use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 50,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // This concept of being concerned only with the messages
    // a value responds to rather than the value's concrete type
    // is similar to duck typing in dynamically typed languages

    // If we try to use a type that doesn't implement the Draw trait
    // then the compiler will throw an error

    //
    // Trait objects perform dynamic dispatch
    //

    // Dynamic dispatch is when the compiler can't tell at compile time
    // which method we're calling, the compiler emits code at that at 
    // runtime will figure out which method to call

    // When we use trait objects, Rust must use dynamic dispatch
    
    // This is relatively slower than static dispatching
}
