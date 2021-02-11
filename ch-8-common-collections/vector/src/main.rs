fn main() {
    let v: Vec<i32> = Vec::new();

    // Vec<T> of type i32
    let v1 = vec![1, 2, 3];

    // Updating vectors
    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    // A vector is freed when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];
    } // v goes out of scope and is freed here

    // Reading elements of vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    // Indexing using [] will panick and throw error if index is out of bounds
    // .get() method will just continue without panicking and return None
    let v = vec![1, 2, 3, 4, 5];

    // This will thrown an error
    // let does_not_exist = &v[100]; 
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);


    // Iterate over the values in a vector
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    // Making changes while iterating (mutable reference)
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        // Dereference to change value in reference
        *i += 50;
    }

    
}