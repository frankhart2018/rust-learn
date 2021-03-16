fn main() {
    // Expressions that don't finish normally are assigned the special type !
    // They are exempt from the rules about types having to match
    // It is a divergent function

    fn _hello() -> ! {
        loop {
            let _x = 1;
        }
    }
}
