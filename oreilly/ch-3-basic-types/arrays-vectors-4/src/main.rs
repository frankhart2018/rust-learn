fn main() {
    // Three types of representing a sequence of values in memory

    // 1. [T; N] represents an array of N values, each of type T
    // The array's size is determined at compile time, and is part of the type
    // New values cannot be appended, the size cannot be shrinked too

    // 2. Vector Vec<T> dynamically allocated, growable sequence of values of type T

    // 3. &[T] (shared slice) and &mut[T] (mutable slice) are references to a series
    // of elements that are part of some other value like array or vector
    // A slice can be thought of as a pointer to the first element, along with the count
    // of number of elements that can be accessed starting from that point

    // ARRAY

    // Directly assigning array
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    // Long filled array
    let mut sieve = [true; 10000]; // Array of size 10000 all assigned to true
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    // In rust there is no notation for an unitialized array

    // The size needs to be known at compile time
    // So, something like [true; n], where n is a variable won't work
    // In this case vector has to be used

    // This will fail
    // let n = 2;
    // let arr: [u8; n] = [1, 2];

    // All array methods are actually implemented for slices
    // But implicit conversion is done from reference of array to slice
    // So we can do this
    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort(); // This takes reference as operand, &mut i32 in this case
    assert_eq!(chaos, [1, 2, 3, 4, 5]);

    // VECTOR

    // Creation using vec! macro
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    // Pushing items
    v.push(11);
    v.push(13);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);

    // Create a vector with n items of same value
    let _v1 = vec!["hello"; 10];

    // Creation of vector using Vec::new()
    let mut v = Vec::new();
    v.push("step");
    v.push("on");
    v.push("no");
    v.push("pets");
    assert_eq!(v, vec!["step", "on", "no", "pets"]);

    // Creation using an iterator
    let v: Vec<i32> = (0..5).collect();
    assert_eq!(v, [0, 1, 2, 3, 4]);

    // Slice methods work with vectors too
    let mut v = vec!["a man", "a plan", "a canal", "panama"];
    v.reverse();
    assert_eq!(v, vec!["panama", "a canal", "a plan", "a man"]);

    // If we know the size of vector from start we can use
    // Vec::from_capacity() instead of Vec::new()
    // Then additions can be done without copying to a larger chunk
    // vec! macro uses this trick since it knows the capacity

    let mut v = Vec::with_capacity(2);
    assert_eq!(v.len(), 0);
    assert_eq!(v.capacity(), 2);

    v.push(1);
    v.push(2);
    assert_eq!(v.len(), 2);
    assert_eq!(v.capacity(), 2);

    v.push(3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.capacity(), 4);

    let mut v = vec![10, 20, 30, 40, 50];

    // Insertion
    v.insert(3, 35);
    assert_eq!(v, [10, 20, 30, 35, 40, 50]);

    // Removal
    v.remove(1); 
    assert_eq!(v, [10, 30, 35, 40, 50]);

    let mut v = vec!["carmen", "miranda"];
    // Returns Option<T> -> None if vector is empty or Some(v)
    assert_eq!(v.pop(), Some("miranda"));
    assert_eq!(v.pop(), Some("carmen"));
    assert_eq!(v.pop(), None);

    // For loop with vector
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }

    // SLICE

    // Slices can be used only as references
    // A reference to a slice is a fat pointer 
    // which is a two word value comprising a pointer to the slice's first element
    // and the number of elements in the slice

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, 0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let _sa: &[f64] = &a;

    // An ordinary reference is a non-owning pointer to a single value
    // whereas a slice is a non-owning pointer to several values
    // This is useful when using homogeneous data series stored in array, vector, stack or heap

    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }

    print(&v);
    print(&a);

    // Reference to slice
    print(&v[0..2]);
    print(&a[2..]);
    print(&sv[1..3]);

    // Rust checks the indices, whether they are valid or not
}
