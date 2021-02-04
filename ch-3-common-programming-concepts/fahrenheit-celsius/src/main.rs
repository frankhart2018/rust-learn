fn f_to_c(f:i32) -> i32 {
    ((f - 32) * 5) / 9
}

fn c_to_f(c:i32) -> i32 {
    ((9 * c) / 5) + 32
}

fn main() {
    let f = 77;
    let c = 5;

    let f_c = f_to_c(f);
    let c_f = c_to_f(c);

    println!("f: {}, c: {}", f, f_c);
    println!("f: {}, c: {}", c_f, c);
}
