struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // Field init shorthand, instead of writing email: email, we can just write email
    User {
        email,
        username, 
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("User(email: {}, username: {}, active: {}, sign_in_count: {})", user.email, user.username, user.active, user.sign_in_count);
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"), 
        String::from("someusername123"),
    );

    // Copying values from another struct variable
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    print_user(&user2);

    // Copying values using struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    print_user(&user2);

    
}