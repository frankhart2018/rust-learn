use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32 { // This trait bound shows that it is a closure
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // Unlike functions, closures can capture values from the scope in which
    // they are defined

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_faster(simulated_user_specified_value, simulated_random_number);
    generate_workout_cached(simulated_user_specified_value, simulated_random_number);

    // Closures can also capture their environment and access variables
    // from the closure's surrounding environment
    let x = 4;

    // This borrows x immutably
    let equal_to_x = |z| z == x; // This can't be done with functions
    // But this capturing of environment has an overhead as the values will be copied in memory
    // This overhead is not possible in functions

    // To take ownership we can add move keyword
    // let equal_to_x = move |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

// This calls simulated_expensive_calculation too many times, some calls can be cut
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

// Faster version of generate_workout
fn generate_workout_faster(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);

    // Closure
    let expensive_closure = |num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Closure with type annotated
    let expensive_closure_with_type = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Closure definitions will have one concrete type inferred for each
    // of their parameters and for their return value

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure_with_type(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result
            );
        }
    }
}

fn generate_workout_cached(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity),
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity),
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity),
            );
        }
    }
}


