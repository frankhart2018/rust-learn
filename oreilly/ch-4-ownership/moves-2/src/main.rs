fn main() {
    // In Rust, for most types, operations like assigning a value to a variable,
    // passing it to a function, or returning it from a function don't copy the value
    // they move it, i.e. the source relinquishes ownership of the value to the destination

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let _t = s; // t now owns the vector, s is now uninitialized and unusable
    // let u = s; // This will throw error as s is uninitialized

    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // value "Govinda" dropped here
    println!("{}", s);

    let mut s1 = "Govinda".to_string();
    let _t = s1; // "Govinda" is now owned by t
    s1 = "Siddhartha".to_string(); // Nothing is dropped, since s1 was already unitializied in previous line

    // Moves in the previous section code
    
    #[allow(unused)]
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525
    });

    // The vall Vec::new() constructs a new vector, and returns, not a pointer to the vector,
    // but the vector itself: its ownership moves from Vec::new() to the variable composers
    // Similarly, the to_string() call returns a fresh String instance

    // The name field of the new Person structure is initialized with the return value of to_string
    // The structure takes ownership of the string

    // The entire Person structure, not just a pointer, is passed to the vector's push method,
    // which moves it onto the end of the structure
    // The vector takes ownership of the Person, and thus becomes the indirect owner of the name String as well

    // MOVES AND CONTROL FLOW
    let x = vec![10, 20, 30];
    if true {
        let _c = x; // Ok to move x from here
    } else {
        let _c = x; // Also ok to move x from here
    }
    // let _c = x; // This will throw error as x is already moved by either if or else

    // For same reasons moving a variable in loop is forbidden
    let x = vec![1, 2, 3];
    while true {
        // let _c = x; // This will throw error as first time it would have been moved
        // x cannot be moved unless it has definitely been given a new value before the next iteration
    }

    // MOVES AND INDEXED CONTENT

    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    // Pull out random elements from vector
    // let third = v[2];
    // let fifth = v[4];

    // For these to work the vector would have to track that index 2 and 4 have been uninitialized
    // That will mean tracking a lot of information until the vector is dropped
    // This would make vectors inefficient thus this is completely banned from Rust compiler
    // To move in this case we can use reference

    // But what if we actually want to move the element?
    // This can be required in the following cases:-

    // 1. Pop a value off the end of the vector
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    // 2. Move a value out of the middle of the vector, and move the last
    // element to its spot
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're talking about
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(v, vec!["101", "104", "substitute"]);

    // Each one of these methods moves an element out of the vector, but does so in
    // a way that leaves the vector in a state that is fully populated, if perhaps smaller

    let v = vec!["libterté".to_string(),
                            "égalité".to_string(),
                            "fraternité".to_string()];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    // When we pass the vector to the loop directly, this moves the vector out of v,
    // leaving v uninitialized, the for loop's internal machinery takes ownership of the vector,
    // and dissects it into its elements, at each iteration the loop moves another element
    // to the variable s, since s now owns the string, we're able to modify it in the loop
    // body before printing it, and since the vector itself is no longer visible to the code, 
    // nothing can observe it mid-loop in some partially emptied state

    #[allow(unused)]
    struct AnotherPerson {
        name: Option<String>,
        birth: i32
    }

    let mut composers = Vec::new();
    composers.push(AnotherPerson {
        name: Some("Palestrina".to_string()),
        birth: 1525
    });

    // let first_name = composers[0].name; // This will throw error
    // But since name is of type Option we can just replace its value by None

    let first_name = std::mem::replace(&mut composers[0].name, None);
    assert_eq!(first_name, Some("Palestrina".to_string()));
    assert_eq!(composers[0].name, None);

    // Since the above approach of using Option and mem::replace is common, vectors have take()
    // in-built method
    let first_name = composers[0].name.take();
}
