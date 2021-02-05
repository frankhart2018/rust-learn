fn main() {
    // let reference_to_nothing = dangle();
    let str = no_dangle();
    println!("{}", str);
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     // s goes out of scope and we are returning a reference to the calling function from here
//     // this is called a dangling reference
//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");

    // In this case ownership is transferred, so no errors
    s
}