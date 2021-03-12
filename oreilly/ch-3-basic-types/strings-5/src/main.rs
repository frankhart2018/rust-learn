fn main() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    // String spanning multiple lines

    // In this case all the newlines and extra spaces will be retained
    println!("In the room the women come and go,
        Singing of Mount Abora");

    // In this case all the newlines and extra spaces will be droppped
    println!("It was a bright, cold day in April, and \
        threre were four of us-\
        more or less.");

    // Raw string, all characters are included verbatim, no escape sequences are recognized
    let default_win_install_path = r"C:\Program Files\Gorillas";
    println!("{}", default_win_install_path);

    // Multi-line raw strings
    println!(r###"
        This raw string stated with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);

    // Byte strings - slide of u8 values rather than unicode text
    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    // Byte strings can span multiple lines, use escape sequences, and use 
    // backslashes to join lines, raw byte strings start with br

    // Common string methods do not work with byte string

    // STRING METHODS

    // len() - Length of string
    assert_eq!("ಠ_ಠ".len(), 7);

    // chars() - Converts to UTF-8 char
    // count() - Counts the number of chars
    assert_eq!("ಠ_ಠ".chars().count(), 3);

    // We cannot modify &str
    let mut _s = "hello"; // This is of type &str, this borrows directly from the executable's location in memory
    // s[0] = 'c'; // Error: the type `str` cannot be mutably indexed
    // s.push('\n'); // Error: no method named `push` found for type `&str`

    // WAYS TO CREATE STRINGS

    // 1. Using to_string() method, this converts &str to String
    let error_message = "too many pets".to_string();
    println!("{}", error_message);

    // 2. Using format!() macro
    assert_eq!(format!("{}°{:02}′{:02}″N", 24, 5, 23), "24°05′23″N".to_string());

    // 3. Using concat() or join() methods
    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(", "), "veni, vidi, vici");

    // Equal and not equal operator
    assert!("ONE".to_lowercase() == "one");

    assert!("peanut".contains("nut"));
    assert_eq!("ಠ_ಠ".replace("ಠ", "^"), "^_^");
    assert_eq!("      clean\n".trim(), "clean");

    for word in "veni, vidi, vici".split(", ") {
        assert!(word.starts_with("v"));
    }
}
