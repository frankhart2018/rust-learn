#[derive(Debug)]
enum Message {
    Quit, // No datatype associated
    Move { x: i32, y: i32 }, // Struct type
    Write(String), // Tuple struct type
    ChangeColor(i32, i32, i32), // Tuple struct type
}

impl Message {
    fn call(&self) {
        println!("Message call method called!");
    }
}

#[derive(Debug)]
struct QuitMessage; // unit struct

#[derive(Debug)]
struct MoveMessage {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct WriteMessage(String); // tuple struct

#[derive(Debug)]
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
enum NewMessage {
    Quit(QuitMessage),
    Move(MoveMessage),
    Write(WriteMessage),
    ChangeColor(ChangeColorMessage),
}

fn main() {
    let msg_1 = Message::Quit;
    let msg_2 = Message::Move { x: 10, y: 10 };
    let msg_3 = Message::Write(String::from("Hello"));
    let msg_4 = Message::ChangeColor { 0: 10, 1: 11, 2: 12 };

    println!("{:#?}", msg_1);
    println!("{:#?}", msg_2);
    println!("{:#?}", msg_3);
    println!("{:#?}", msg_4);

    msg_1.call();

    let quit_message = QuitMessage {};
    let move_message = MoveMessage {
        x: 10,
        y: 10
    };
    let write_message = WriteMessage {
        0: String::from("Hello"),
    };
    let change_color_message = ChangeColorMessage {
        0: 10,
        1: 11,
        2: 12,
    };

    let msg_1 = NewMessage::Quit(quit_message);
    let msg_2 = NewMessage::Move(move_message);
    let msg_3 = NewMessage::Write(write_message);
    let msg_4 = NewMessage::ChangeColor(change_color_message);

    println!("{:#?}", msg_1);
    println!("{:#?}", msg_2);
    println!("{:#?}", msg_3);
    println!("{:#?}", msg_4);
}