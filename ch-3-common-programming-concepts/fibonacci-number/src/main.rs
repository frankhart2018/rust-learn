fn fibonacci(n:i32) -> i32 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("Fibbonaci of 9 = {}", fibonacci(9));
}
