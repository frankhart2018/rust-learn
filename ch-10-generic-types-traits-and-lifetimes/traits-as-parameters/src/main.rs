#![allow(unused)]
fn main() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    
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
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    // trait as a parameter - this can be called from any instance of NewsArticle or Tweet
    // Types like String or i32 cannot be used to call this as they don't implement Summary
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // trait as a parameter - trait bound syntax
    // the above method is a syntax sugar over this
    pub fn notify_again<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Multiple trait bounds using + syntax
    // pub fn notify(item: &(impl Summary + Display))
    // or
    // pub fn notify<T: Summary + Display>(item: &T)

    // Clearer trait bounds
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U)
    // can be written as
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    //      where T: Display + Clone,
    //            U: Clone + Debug 

    // Returning traits that implement traits, helps in returning any type that implements this trait
    // But this can return either NewsSummary or Tweet not both (using if-else or switch)
    fn return_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
