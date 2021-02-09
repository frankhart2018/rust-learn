mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// All names brought into scope are still private until pub is prepended to them

// Absolute path
// use crate::front_of_house::hosting;

// Relative path
// use self::front_of_house::hosting;

// This makes it unclear during usage (from where did add_to_waitlist come?)
// But it is a good idea when bringing structs, enums, etc to path to use the name too
// use crate::front_of_house::hosting::add_to_waitlist;

// As keyword
use crate::front_of_house::hosting as host;

// use std::cmp::Ordering;
// use std::io;

// These lines can be shortened to:-

// use std::{cmp::Ordering, io};

// And for these:-

// use std::io;
// use std::io::Write;

// The short form is:-

// use std::io::{self, Write};

// Glob operator
use std::collections::*;

pub fn eat_at_restaurant() {
    host::add_to_waitlist();
    host::add_to_waitlist();
    host::add_to_waitlist();
}