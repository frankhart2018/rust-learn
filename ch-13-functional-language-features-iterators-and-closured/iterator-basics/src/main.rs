use std::vec;

fn main() {
    // In Rust iterators are lazy, which means that
    // they don't perform the computation until the method that
    // consumes the iterator is called

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // This doesn't do anything at this point

    for val in v1_iter {
        println!("Got: {}", val); // Now iter is consumed
    }

    //
    // The Iterator trait and the next method
    //

    // All iterators implement a trait called Iterator from standard library

    let mut v1_another_iter = v1.iter(); // Using .next() changes internal state of this, hence this needs
                                                  // to made mutable

    assert_eq!(v1_another_iter.next(), Some(&1));
    assert_eq!(v1_another_iter.next(), Some(&2));
    assert_eq!(v1_another_iter.next(), Some(&3));
    assert_eq!(v1_another_iter.next(), None);

    //
    // Methods that consume the iterator
    //

    // Methods that call next are called 'consuming adaptors' because calling
    // them uses up the iterator

    //
    // Methods that produce other iterator
    //

    // Other methods that are defined on the Iterator trait, are known as iterator adaptors
    // these method calls allow us to change iterators into different kinds of iterators

    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1); // This does nothing, as it does lazy evaluation
    // Computation will be done only when this is consumed

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum(); // v1_iter won't be usable after this
        // as sum calls next and consumes the iterator

        assert_eq!(total, 6);
    }
}
