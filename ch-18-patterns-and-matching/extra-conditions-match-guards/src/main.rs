
fn main() {
    // A match guard is an additional if condition specified
    // after the pattern in a match arm that must also match along
    // with the pattern matching for that arm to be chosen

    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Using match guards to prevent shadowing

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n), // If we used Some(y) then it would have shadowed outer y
        _ => println!("Default case, x = {:?}", x),
    }

    println!("At the end: x = {:?}, y = {}", x, y);

    // Using | (or) operator in a match guard

    let x = 4;
    let y = false;

    match x {
        // The precedence looks like (4 | 5 | 6) if y rather than 4 | 5 | (6 if y)
        4 | 5 | 6 if y => println!("Yes"), // If x = 4 or 5 or 6 and y = true
        _ => println!("No")
    }
}
                