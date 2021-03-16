fn main() {
    // Other ways to introduce dangling pointers:-
    let v = vec![4, 8, 19, 27, 34, 10];
    let _r = &v;
    // let aside = v; // v is uninitialized here
    // r[0]; // Uses v which is now uninitialized
}
