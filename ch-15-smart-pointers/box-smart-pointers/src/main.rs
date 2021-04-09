fn main() {
    // Boxes allow us to store data in the heap instead of the stack
    // The pointer to the heap data remains in stack 

    // This can be used in following situations:-

    // 1. When we have a type whose size can't be known at compile time and we want to use 
    // a value of that type in a context that requires an exact size

    // 2. When we have a large amount of data and we want to transfer ownership but ensure
    // the data won't be copied when we do so

    // 3. When we want to own a value and we care only that it's a type that implements
    // a particular trait rather than being of a specific type

    //
    // Syntax of Box<T>
    //

    let b = Box::new(5);
    println!("b = {}", b);

    // Deallocation is done for both the pointer in stack and the data in heap when
    // 'b' goes out of scope

    // 
    // Enabling Recursive Types with Boxes
    //

    // The compiler cannot know the size of a type which has a value of the same type
    // this can go on infinitely (in theory)
    // Since, boxes have defined size, so by inserting a box in a recursive type definition
    // we can have recursive types

    //
    // Cons list
    //

    // A cons list is a data structure that comes from Lisp and its dialects
    // In Lisp, cons function (construct function) constructs a new pair from its two
    // arguments, which usually are a single value and anothe pair, these pairs
    // containing pairs form a list

    // Each item in a cons list contains two elements - the value of the current item
    // and the next item
    // The last item contains a value called Nil without a next item

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}", list);

    // Box<T> is a smart pointer because it implements the Deref trait
    // which allows Box<T> values to be treated like references

    // When a Box<T> value goes out of scope the value in the heap is 
    // also cleaned because it inherits Drop trait too
}
