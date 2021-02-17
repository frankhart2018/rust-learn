// std::cmp::PartialOrd needs to be specified in the traits bound
// so that the function can work on slices of any type

// Copy is added to the traits for the assignment of largest
// this ensures that this function will work only with types that implement copy trait

// If we don't want to restrict we can use Clone instead of Copy which will transfer
// ownership of the data to this function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
