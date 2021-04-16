
fn main() {
    let v = vec!['a', 'b', 'c'];

    // In the statement for x in y, x is the pattern

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
                