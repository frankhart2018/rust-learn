fn five() -> i32 {
    5
}

fn main() {
    another_function(5, 6);

    // Expression
    let x = 5;
    println!("x: {}", x);
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// Rust doesn't care whether you define it above the code where it is called or below it, it just needs to be defined
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
