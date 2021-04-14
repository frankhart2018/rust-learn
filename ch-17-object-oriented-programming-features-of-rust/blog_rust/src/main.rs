use blog_rust::Post;

// Encoding state in the type system itself to get
// errors during compilation
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content()); // This won't work now because all Post starts with DraftPost 
                                       // which doesn't have a content method

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
