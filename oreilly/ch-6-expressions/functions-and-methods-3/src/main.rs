fn main() {
    // This won't work
    // return Vec<i32>::with_capacity(1000);
    // This is because in expressions < is less than operator
    // We can use this
    // Vec::<i32>::with_capacity(1000);

    // The symbol ::<..> is called turbofish in Rust community

    // Alternatively we can drop the type parameter and let rust figure out the type
}
