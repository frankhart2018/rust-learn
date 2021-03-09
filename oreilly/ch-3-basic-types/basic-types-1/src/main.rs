fn main() {
    // This will panic as arithmetic operation overflowed
    // But in release mode it will wrap over
    let big_val = std::i32::MAX;
    // let x = big_val + 1;

    // Explicit wrapping
    let x = big_val.wrapping_add(1); 
    println!("x = {}", x);

    // TYPE CONVERSION - INTEGER

    // In rust there is no implicit type conversion
    // All types have to be converted explicitly
    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(2525_u16 as i16, 2525_i16); // in range

    assert_eq!(-1_i16 as i32, -1_i32); // sign extended
    assert_eq!(65535_u16 as i32, 65535_i32); // zero extended

    // Conversions that are out of range for the destination
    // produce values that are equiavalent to the original modulo 2^N,
    // where N is the width of the destination in bits. This is sometimes
    // called "trunacation"
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b101101u8.count_ones(), 4);

    // TYPE CONVERSION - FLOAT

    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.);
    assert_eq!((-1.01f64).floor(), -2.0);
    assert!((-1. / std::f32::INFINITY).is_sign_negative());

    // This will throw an error as sqrt is not defined for float
    // println!("{}", (2.0).sqrt());

    println!("{}", (2.0_f64).sqrt());
    println!("{}", f64::sqrt(2.0));

    // TYPE CONVERSION - BOOL

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    // Converting from i32 is not allowed in rust
    // This is why we have to specify the entire boolean expression
    // Like if x != 0 and not just if x

    // TYPE CONVERSION - CHAR

    assert_eq!('*' as i32, 42);
    assert_eq!('ಠ' as u16, 0xca0);
    assert_eq!('ಠ' as i8, -0x60); // U+0CA0 truncated to eight bits, signed

    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('β'.is_alphabetic(), true);
    assert_eq!('8'.to_digit(10), Some(8)); // to_digit() takes the base of number system as input
    assert_eq!('ಠ'.len_utf8(), 3);
    assert_eq!(std::char::from_digit(2, 10), Some('2')); 
}
