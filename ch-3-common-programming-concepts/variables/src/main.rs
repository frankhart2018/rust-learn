fn main() {
    // Shadowing
    let x = 5;
    
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "    "; // Will throw an error if it were mut, as we cannot change type of mutable variable
    let spaces = spaces.len();

    println!("The number of spaces: {}", spaces);

    // Floating point type
    let float_x = 2.0; // f64 - equivalent to double

    let float_y: f32 = 3.0; // f32 - equivalent to float

    println!("x: {}, y: {}", float_x, float_y);

    // Arithmetic operators

    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    // Boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("t: {}, f: {}", t, f);

    // Char type
    let c = 'z';
    println!("c: {}", c);
    let c = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c: {}, heart_eyed_cat: {}", c, heart_eyed_cat);

    // Tuple (compound type)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup[0] = {}, tup[1] = {}, tup[2] = {}", tup.0, tup.1, tup.2);

    // Destructuring tuple
    let (x, y, z) = tup;
    println!("x: {}, y: {}, z: {}", x, y, z);

    // Indexing tuple
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    // Array (compound type) - In Rust array is like tuple but containing data of same type
    let a = [1, 2, 3, 4, 5];

    // Array indexing
    println!("[{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October",
                  "November", "December"];
    println!("{} - {}", months[0], months[11]);

    // Type annotated array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[0] = {}", a[0]);

    // Initializing array with same values
    let a = [3; 5];
    println!("[{}, {}, {}, {}, {}]", a[0], a[1], a[2], a[3], a[4]);
}