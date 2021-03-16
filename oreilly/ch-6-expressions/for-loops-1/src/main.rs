fn main() {
    // Simple for loop
    for i in 0..20 {
        println!("{}", i);
    }
    
    // 0..20 is same as using std::ops::Range { start: 0, end: 20 }

    // Loop with lifetime label
    'search:
    for i in 0..20 {
        for j in 0..3 {
            if i == 2 {
                break 'search; // This breaks out not only of the immediate loop but the outer loop 
            }
            println!("{}->{}", i, j);
        }
    }

    // Labels can also be used with continue
}
