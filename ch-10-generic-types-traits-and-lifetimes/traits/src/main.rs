#![allow(unused)]
fn main() {
    // Without default implementation
    // pub trait Summary {
    //     fn summarize(&self) -> String;
    // }

    // Default implementation
    pub trait Summary {
        // This needs to be implemented in order to use summarize_something
        fn summarize_author(&self) -> String;

        // Default implementation can use other implementations even if they are not defined
        fn summarize_something(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }

        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    // Implement the trat
    impl Summary for NewsArticle {
        // fn summarize(&self) -> String {
        //     format!("{}, by {} ({})", self.headline, self.author, self.location)
        // }
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

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguings with the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    // Using default implementation
    println!("New article available! {}", article.summarize());
}

// For trait implementation either the trait or type or both needs to be in local scope
// We cannot implement external trait on an external type
// This rule is called coherence or orphan rule (since the parent type is not present)
// This ensures other people's code don't break when used as crates