fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// This can be fixed with trait bounds
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest { // This throws error because we cannot do > for all possible types T
//             largest = item;
//         }
//     }

//     largest
// }

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

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'c', 'a'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
