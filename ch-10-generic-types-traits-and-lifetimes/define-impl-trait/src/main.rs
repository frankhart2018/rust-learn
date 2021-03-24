use std::fmt::Display;

#[allow(unused)]
fn main() {
    // Trait definitions are a way to group method signatures
    // together to define a set of behaviours necessary to accomplish some purpose

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // Implementing a trait on a type
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    };

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    };

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // If someone else wants to use Summary trait they will have to bring it in their scope
    // Using -> use aggregator::Summary (considering this lib crate's name is aggregator
    
    // We cannot implement external traits on external types
    // For e.g.:- We can't implement Display trait on Vec<T> within our aggregator crate
    // Because Display and Vec<T> are defined in the standard library and aren't local to aggregator crate

    // Default implementation
    pub trait DefSummary {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // To use this we need to create an empty impl block
    impl DefSummary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."
        ),
    };

    println!("New article available! {}", article.summary());

    // To use this, we will have to implement summarize_author
    pub trait AnotherSummary {
        fn summarize_author(&self) -> String;

        fn another_summary(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    impl AnotherSummary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.another_summary());

    // Traits as parameters

    // This function can only be called by types that implement Summary trait
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Trait bound syntax for trait parameters
    pub fn notify_again<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Mutliple parameters
    // pub fn notify(item1: &impl Summary, item2: &impl Summary)
    // or in trait-bound form
    // pub fn notify<T: Summary>(item1: &T, item2: &T)
    
    // Specifying multiple trait bounds using + syntax
    // pub fn notify(item: &(impl Summary + Display)) 
    // or in trait-bound form
    // pub fn notify<T: Summary + Display>(item: &T) 

    // Clearer trait bounds with where clauses
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
    // This makes the function signature hard to read, here we can use where clause
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //      where T: Display + Clone
    //            U: Clone + Debug

    // Returning types that implement traits
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    // By specifying impl Summary as the return type we specify that
    // returns_summarizable returns some type that implements Summary trait without naming the concrete type
    // But this works only if we are returning a single type, it will fail if there are multiple
    // concrete types returned even if they implement Summary trait

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x, y
            }
        }
    }


    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) { // This function can only be called by those instances of Pair that implement either traits
            if self.x >= self.y {
                println!("The largest number is x = {}", self.x);
            } else {
                println!("The largest number is y = {}", self.y);
            }
        }
    }

    // Conditional implementation of a trait for any type that implements another trait
    // impl<T: Display> ToString for T {
    // This trait can be implemented only for those types that already implement Display trait
    // These are called blanket implementations
}
