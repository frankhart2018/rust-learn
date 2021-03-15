fn main() {
    // Unlike C++, in Rust references are created explicitly with the & operator
    // and dereferenced explicitly using * operator
    let x = 10;
    let r = &x; // &x is a shared reference to x
    assert!(*r == 10); // explicitly dereference r

    // Mutable reference
    let mut y = 32;
    let m = &mut y; // &mut y is a mutable reference to y
    *m += 32; // explicitly dereference m to set y's value
    assert!(*m == 64); // and to see y's new value

    // * operator implicitly dereferences its left operand if needed

    struct Anime {
        name: &'static str,
        bechdel_pass: bool
    };

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!(anime_ref.bechdel_pass, true);

    // Equivalent to the above, but with the dereference written out
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // Implicit borrowing of reference
    let mut v = vec![1973, 1968];
    v.sort(); // implicitly borrows a mutable reference to v
    (&mut v).sort(); // equivalent as above

    // Assigning to a Rust reference makes it point at a new value
    let x = 10;
    let y = 10;
    let b = true;
    let mut r = &x;

    if b {
        r = &y; // if b is true then r will point to y
    }

    assert!(*r == 10 || *r == 20);

    // References to references

    #[allow(unused)]
    struct Point {
        x: i32,
        y: i32,
    };

    let point = Point {
        x: 1000, 
        y: 729,
    };

    let r: &Point = &point;
    let rr: &&Point = &r;
    let rrr: &&&Point = &rr;
    assert_eq!(rrr.y, 729);

    // Comparing references
    // Like the . operator, Rust's comparison operators "see through" any number of references,
    // as long as both operands have same type

    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    assert!(rrx <= rry);
    assert!(rrx == rry);

    // This succeeds because comparison operators follow all the references and perform 
    // the comparison on their final targets, x and y

    // Rust references are never null
    // There is no NULL as in C or nullptr as in C++
    // To use null pointer we can use Option<&T> which returns None or Some(r)

    // Borrowing references to arbitrary expressions
    // In C and C++ we can apply the & operator to certain kinds of expressions
    // Rust on the other hand allows us to borrow a reference to the value of any sort of expression 

    fn factorial(n: usize) -> usize {
        (1..n+1).fold(1, |a, b| a * b)
    }

    let r = &factorial(6);
    assert_eq!(r + &1009, 1729);

    // In these cases Rust simply creates an anonymous variable to hold the expression's value
    // and makes the reference point to that

    // The lifetime of this anonymous variable depends on what we do with the reference:

    // 1. If we immediately assign the reference to a variable in a let statement 
    // (or make it part of some struct or array that is being immediately assigned),
    // then Rust makes the anonymous variable live as long as the variable the let initializes
    // In the preceeding example, Rust would do this for the referent of r

    // 2. Otherwise, the anonymous variable lives to the end of the enclosing statement
    // In our example, the anonymous variable created to hold 1009 lasts only to the end of the assert_eq! statement
}
