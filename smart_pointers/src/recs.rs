// reference counters allow for multiple ownership
// Rc<T> keeps track of the number of references to a value which determines if the value is in use
// Rc<T> is only used in single threaded scenarios

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil
}

use crate::recs::List::{Cons, Nil};
use std::rc::Rc;

pub fn rc_stuff() {
    let one = Rc::new(Cons(false, Rc::new(Cons(true, Rc::new(Nil)))));
    // Rc clone increments the reference counter by one, it doesn't do a deep copy of the data
    // both two and three own one
    let two = Cons(false, Rc::clone(&one));
    let three = Cons(true, Rc::clone(&one));
    {
        let four = Cons(false, Rc::clone(&one));
        println!("Number of owners of one: {}", Rc::strong_count(&one));
    }
    // four has gone out of scope so the counter on the Rc has decreased by one
    println!("Number of owners of one: {}", Rc::strong_count(&one));
}

// RefCell<T> allows you to mutate data even when there are immutable references to it
// RecCell uses unsafe code
// mutating the value inside an immutable value is the interior mutability pattern

// trait definition, specifies what methods need to be defined for a type to implement a trait
pub trait Messenger {
    fn send(&self, msg: &str);
}

// the generic T must implement the trait messenger
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

// again, specifying that T must implement Messenger
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    // constructor, sets base values
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    // changes self.value and uses that value to check against max
    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: you've exceeded your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: you've used up over 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: you've used up over 75% of your quota");
        }
    }
}

// want to test the set_value functionality using a mock object
#[cfg(test)]
mod tests {
    use super::*;
    // refcell keeps track of the number of Ref<T> and RefMut<T> smart pointers currently active
    // while the borrow checker prevents compiling if the ownership rules are broken,
    // refcell will panic if ownership rules are broken using refcells at runtime
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // borrow_mut is refcell's way of creating mutable references
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
