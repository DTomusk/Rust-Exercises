// in unsafe rust you can:
// dereference raw pointers
// call unsafe functions and methods
// access or modify a mutable static variable
// implement unsafe traits
// access fields of union S

mod traits;
mod types;
mod adfunc;
mod macaroni;

use std::slice;

// static variables are rust's version of global variables
// static variables have 'static lifetimes
// accessing and modifying static variables is unsafe
static BAD: &str = "He actually turned himself into a pickle";
static GOOD: i32 = 12;

fn main() {
    println!("Hello, world!");

    let mut me = 3;

    // creating raw pointers, raw pointers can ignore borrowing rules, are allowed to be null, don't implement
    // automatic clean up and aren't guaranteed to point to valid memory
    // we can have immutable and mutable pointers to the same data with raw pointers
    let this = &me as *const i32;
    let that = &mut me as *mut i32;

    // creating raw pointers is safe but dereferencing them isn't
    unsafe {
        println!("this is: {}", *this);
        println!("that is: {}", *that);

        // unsafe functions must be called from within unsafe blocks
        //bread();

        println!("The absolute value of -7 according to C is {}", abs(-7));
    }

    // printing static variables is safe
    println!("{}", BAD);

    traits::trait_stuff();

    adfunc::funky();
}

// you take responsibility for calling unsafe functions
unsafe fn bread() {
    let addr = 0x069420usize;
    let r = addr as *const i32;
    println!("Address: {}", *r);
}

// this function is a safe abstraction of our unsafe code and can be called from safe rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    // returns the raw pointer of the slice
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // we've wrapped unsafe code inside a safe function
    unsafe {
        (
            // slices consist of a pointer to data and a length
            // from_raw_parts_mut is unsafe because it has to trust that the given parameters are valid
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// extern lets us create foreign function interfaces
// FFIs allow us to call functions from a different language
// "C" is the application binary interface for this function
// the ABI defines how to call the function at the assembly level
extern "C" {
    fn abs(input: i32) -> i32;
}
