use blog::Post;

fn main() {
    // State pattern is an OOP pattern, the crux of this is that
    // a value has some internal state which is represented by a 
    // set of state objects and the value's behaviour changes
    // based on the internal state

    // Each state object is responsible for its own behaviour 
    // and for governing when it should change into another state

    // The value that holds a state object knows nothing about
    // the different behaviour of the states or when to
    // transition between states

    // We'll implement a blog post workflow in an incremental way
    // The blog's final functionality will look like this:-

    // 1. A blog post stats as an empty draft

    // 2. When the draft is done, a review of the post is requested

    // 3. When the post is approved it gets published

    // 4. Only published blog posts returns content to print, so unapproved
    // posts can't accidentally be published

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
