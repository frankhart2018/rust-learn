use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // For values with Copy trait like i32 the values will be copied to hash map, but the ones with
    // ownership like String are moved and ownership is transferred to hash map
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Accessing values in a hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // It returns Option<&V>, if there is no value it will return None

    // Iterating over values in hash map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a hash map

    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10); // This will be overwritten
    scores.insert(String::from("Blue"), 25);
    
    println!("{:?}", scores);

    // Only inserting a value if the key has no value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
