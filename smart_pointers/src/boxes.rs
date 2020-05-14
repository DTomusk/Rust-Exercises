/*
Boxes store data on the heap rather than on the stack
The stack stores a pointer to the box
Boxes are used for:
    types with unknown size at compile time
    transferring ownership of large amounts of data without copying
    owning values with generic types that implement specific traits
*/

use std::ops::Deref;

// recursive data type using boxes
// Box<T> implements the deref trait which allows its values to be treated as references
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

// Box<T> is defined as a tuple struct with one element, so so is our version
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref requires us to implement one method deref
impl<T> Deref for MyBox<T> {
    type Target = T;

    // returns reference to inner data
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("My box is dropping");
    }
}

// bring the variants of the enum into scope so we don't have to say List::Cons or List::Nil
use crate::boxes::List::{Cons, Nil};

pub fn boxodes() {
    let _list = Cons(String::from("Hello"), Box::new(Cons(String::from("What"), Box::new(Nil))));
    let m = MyBox::new(String::from("Adam raised a Cain"));
    let n = MyBox::new(String::from("Churdle Burdle"));
    // these two are equivalent calls, rust uses deref coercion to get an &str from MyBox
    hello(&m);
    hello(&(*n)[..]);
}

pub fn hello(words: &str) {
    println!("The words are: {}", words);
}

#[cfg(test)]
mod test {
    use super::MyBox;

    #[test]
    fn deref() {
        // storing a boolean on the heap (not a good idea)
        let b = Box::new(true);
        // because Box implements deref we can use the * operator to access the data it's pointing to
        assert!(*b);
    }

    #[test]
    fn custom_deref() {
        let m = MyBox::new(24);
        // because MyBox implements deref we can dereference MyBox like a normal reference
        assert_eq!(*m, 24);
    }
}
