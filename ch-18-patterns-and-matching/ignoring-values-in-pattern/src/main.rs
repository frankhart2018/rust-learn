
fn main() {
    //
    // Ignoring an entire value with _
    //

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }

    foo(3, 4);

    // 
    // Ignoring parts of a value with nested _
    //

    // Business case:- User shouldn't be allowed to change value, but can set if its None

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting is: {:?}", setting_value);

    // Underscores in multiple places

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth);
        }
    }

    //
    // Ignoring an unused variable by starting its name with _
    //

    // Difference between _ and _<var-name> is that the first one
    // doesn't bind the value to any variable, the second one binds it
    // it is just that it doesn't throw a warning when the variable isn't used

    //
    // Ignoring remaining parts of a value with ..
    //

    #[allow(unused)]
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x) // match only x, the long form would be
                                                      // Point { x, y: _, z: _ }
    }

    // The .. will expand to as many values as it needs to be

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // The .. shouldn't be ambiguous, like (.., second, ..) this could be any of
    // second third or fourth, and Rust will throw an error in this case
}
                