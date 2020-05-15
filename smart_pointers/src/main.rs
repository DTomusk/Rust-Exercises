mod boxes;
mod recs;
mod leaks;
mod weak_tree;

fn main() {
    println!("Hello, world!");
    recs::rc_stuff();
    boxes::boxodes();
    //leaks::leaks();
    weak_tree::treesauce();
}
