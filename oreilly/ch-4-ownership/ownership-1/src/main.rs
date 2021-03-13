fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // Allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // padovan dropped here, the variable is out of scope so the vector is also dropped

fn main() {
    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // point allocated here
        let label = format!("{:?}", point); // label allocated here
        assert_eq!(label, "(0.625, 0.5)");
    } // both dropped here and so are the heap allocations done by them

    // Just as variables own their values, structs own their fields;
    // and tuples, arrays, and vectors own their elements

    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person { 
        name: "Palestrina".to_string(), 
        birth: 1525 
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    // The ownership relationship here are:-
    // composers owns a vector
    // the vector owns its elements, each of which is a Person structure
    // each structure owns its fields
    // and the string field owns its text
    
    // Rust's way of deleting isn't actually freeing the memory like in C/C++
    // rather it just drops that value from the ownership tree
}
