fn main() {
    // Borrowing a local variable

    {
        let _r  = 1;
        {
            let _x = 1;
            // r = &x; // This is not possible as x is local and its scope is in this local block only
        }
        // assert_eq!(*r, 1); 
    }

    // Lifetime is some stretch of program for which a reference could be safe to use

    // Receiving references as parameters

    // static mut STASH: &i32;
    // fn f(p: &i32) {
    //     unsafe {
    //         // STASH = p;
    //     }
    // }

    // static is Rust's equivalent of global variable

    // Every static must be initialized

    // Mutable statics are inherently not thread-safe, so mutable static is allowed only inside unsafe block

    // We will have to pass static lifetime reference
    static mut STASH: &i32 = &10;

    fn f(p: &'static i32) {
        unsafe {
            STASH = p;
        }
    }

    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);

    // Returning references

    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if *r < *s {
                s = r;
            }
        }
        s
    }

    // let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        // s = smallest(&parabola);

        // This will work
        let s = smallest(&parabola);
        assert_eq!(*s, 0); // This will work though
    }
    // assert_eq!(*s, 0); // This will throw error as parabola is dropped and in smallest function they should have same 
                         // lifetime, in the call, the argument must not outlive parabola itself, which is not the case

    // Structs containing references
    // #[allow(unused)]
    // struct S {
    //     r: &i32,
    // };

    // let s;
    // {
    //     let x = 10;
    //     s = S {
    //         r: &x
    //     };
    //     assert_eq!(*s.r, 10); // This will work
    // }
    // assert_eq!(*s.r, 10); // This will throw error as x which r was referring to is out of scope

    // Whenever a reference type appears inside another type's definition
    // we must write out its lifetime
    // struct S1 {
    //     r: &'static i32
    // };
    // This says that r can only refer to i32 values that will last for the lifetime of the program

    // Instead we can do this
    // struct S<'a> {
    //     r: &'a i32
    // };

    // Now S will have a lifetime just as reference types do
    // Each value we create of type S gets a fresh lifetime 'a 
    // which becomes constrained by how we use the value

    // If we use a type with lifetime parameter inside other type
    // struct T {
    //     // s: S // This will throw error
    //     s: S<'static> // We always need to specify the lifetime parameter in this case
    // };

    // Another approach will be to to give T its own lifetime parameter
    // struct T<'a> {
    //     s: S<'a>
    // }

    // Even types like i32 and String have lifetime, most have 'static as their lifetime
    // meaning that values of these types can live for as long as we like

    // Distinct lifetime parameters
    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32
    // };

    // let x = 10;
    // let r;
    // {
    //     let y = 20;
    //     {
    //         let s = S {
    //             x: &x,
    //             y: &y
    //         };
    //         r = s.x;
    //     }
    // }

    // This above code will fail because:-
    
    // 1. Both x and y are references with same lifetime 'a
    // 2. We assign r = s.x requiring 'a to enclode r's lifetime
    // 3. We initialized s.y with &y, requiring 'a to be no longer than y's lifetim
    // These constraints are impossible to satisfy as no lifetime is shorter than y's scope
    // but longer than r's

    // In this case there should be two lifetime parameters
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32
    };

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S {
                x: &x,
                y: &y
            };
            r = s.x;
            println!("{}", r);
            println!("{}", s.y);
        }
    }
}
