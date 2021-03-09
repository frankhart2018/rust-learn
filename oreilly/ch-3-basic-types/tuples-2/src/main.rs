fn main() {
    // TUPLES

    // Cleaner way
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21); // Here 21 is the index 
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // Longer way
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0; // tuple indexing
    let tail = temp.1;
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}
