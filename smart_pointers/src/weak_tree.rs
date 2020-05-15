use std::cell::RefCell;
use std::rc::{Rc, Weak};

// stores its own value and a list of children
// want multiple ownership of children (tell that to my wife) so both parents and variables can access them
// Rc gives multiple ownership and RefCell gives inner mutability
#[derive(Debug)]
struct Node {
    value: i32,
    // a children knows its parent but doesn't own it, if a children is dropped its parent isn't
    // weak prevents reference cycles
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn treesauce() {
    // leaves have no children so children is an empty vector in a RefCell
    let leaf = Rc::new(Node {
        value: 2,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // we can now access leaf directly and through the branch as both have ownership
    let branch = Rc::new(Node {
        value: 62,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // take a reference to branch, downgrade it to weak and mutably lend that to leaf as its parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("{:?}", leaf);
}
