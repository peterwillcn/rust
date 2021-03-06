// Test that `try!` macros are rewritten.

// run-rustfix
// check-pass

#![warn(rust_2018_compatibility)]
#![allow(dead_code)]
#![allow(deprecated)]

fn foo() -> Result<usize, ()> {
    let x: Result<usize, ()> = Ok(22);
    try!(x);
    //~^ WARNING `try` is a keyword in the 2018 edition
    //~| WARNING this was previously accepted
    Ok(44)
}

fn main() { }
