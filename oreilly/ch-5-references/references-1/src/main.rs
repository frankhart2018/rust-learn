use std::collections::HashMap;

fn main() {
    // Rust refers to creating a reference to some value as 
    // borrowing the value: what you have borrowed, you must eventually return to its owner

    type Table = HashMap<String, Vec<String>>; // Similar to typedef

    fn show(table: Table) {
        for (artist, works) in table { // takes ownership of table
            println!("works by {}:", artist);
            for work in works { // takes ownership of the vector
                println!("    {}", work);
            }
        }
    }

    let mut table = Table::new();
    table.insert("Gseualdo".to_string(), vec!["many madrigals".to_string(),
                                                   "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(),
                                                     "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(),
                                                  "a salt cella".to_string()]);
    
    show(table); // table is moved

    // The proper way to do this is using references

    // There are two references in Rust:-
    // 1. Shared reference (&T) - there can be multiple shared reference at once, these are Copy, can only read value
    // 2. Mutalbe reference (&mut T) - there can be only a single mutable reference at once, these are not Copy, can 
    // both read and write value

    // When a mutable reference is modifying the value even the owner cannot change the value

    // Shared reference example

    let mut table = Table::new();
    table.insert("Gseualdo".to_string(), vec!["many madrigals".to_string(),
                                                   "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(),
                                                     "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(),
                                                  "a salt cella".to_string()]);

    fn borrowed_show(table: &Table) {
        for (artist, works) in table { // takes shared reference of table
            println!("works by {}:", artist);
            for work in works { // takes shared reference of the vector
                println!("    {}", work);
            }
        }
    }

    borrowed_show(&table);

    // Mutable reference example
    
    fn sort_works(table: &mut Table) {
        for (_artist, works) in table {
            works.sort();
        }
    }

    sort_works(&mut table);
}
