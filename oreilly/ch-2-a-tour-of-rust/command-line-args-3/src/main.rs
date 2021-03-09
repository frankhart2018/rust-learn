use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    let mut numbers = Vec::new(); // Vectors also have to be made mutable explicitly to push data to its end

    // skip(1) skips the 1st CLA which is the name of the program
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        // unwrap checks if there is an error while throwing the error message, expect could also
        // have been used instead of unwrap here
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap(); 
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m)
    }

    println!("The greatest commond divisor of {:?} is {}", numbers, d);
}
