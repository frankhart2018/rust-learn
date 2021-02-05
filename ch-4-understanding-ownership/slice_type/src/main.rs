fn main() {
    let mut s = String::from("hello");

    let word = first_word(&s); // word will get the value 5
    println!("{}", word);

    s.clear();

    // Once s is cleared word still has 5, but there is no way we can use that

    // The solution is string slices
    let s = String::from("hello world");

    let hello = &s[0..5]; // References a portion of the string, ending index isn't included just like python
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    // Internally rust stores the pointer to the location and the length
    // To start from 0, we can write [..5], for last [6..], for entire string [..]

    // Improved first word function
    let s = String::from("hello world");

    let word = first_word_improved(&s);
    println!("{}", word);

    // String literals are basically &str (string slices) which point to specific point of binary

    // Array slicing
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("[{}, {}]", slice[0], slice[1]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn first_word_improved(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}