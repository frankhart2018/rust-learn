fn main() {
    // Instead of creating strong reference to Rc<T>
    // weak references can also be created by calling Rc::downgrade
    // and passing a reference to Rc<T>

    // When we call Rc::downgrade it returns a smart pointer of type 
    // Weak<T>, this decreases the weak_count instead of strong_count

    // The Rc<T> uses weak_count to keep track of Weak<T> references
    // similar to strong_count

    // The difference is that weak_count doesn't need to be 0
    // for Rc<T> instance to be cleaned up

    // Weak references don't express an ownership relationship
    
    // They won't cause a reference cycle because any cycle
    // will be broken once the storng reference count is 0

    // To do anything with Weak<T> we have to make sure it exists
    // because it might have been dropped
    // This is done using upgrade method which returns Option<Rc<T>>

    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,  // This is made a Weak reference because making this Rc will create reference cycle
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "Branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "Leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("Leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "Leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
