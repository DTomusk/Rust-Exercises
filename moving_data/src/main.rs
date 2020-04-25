mod slice;

fn main() {
    let this = noncopy_ownership();
    let (x, y) = copy_ownership();
    println!("x: {}", x);
    println!("y: {}", y);

    let mut that = take_and_give(this);

    println!("{}", that);

    // instead of moving the value the function is given a reference to it so it can read the string
    borrow_string(&that);

    // that doesn't go anywhere so you can still use it and mutate it afterwards
    that.push_str(" large celebration!");
    // everything drops out of scope

    // using a mutable reference allows you to change a variable without taking ownership of it
    // you can only have one mutable reference to a variable at a time so clashes don't occur
    change_string(&mut that);

    println!("{}", that);

    let big = & that;
    let small = & that;

    // can have multiple immutable references to the same variable
    println!("{}", big.capacity());
    println!("{}", small.len());

    let _those = that.clone();
    // clear takes a mutable reference to the string 
    that.clear();
    println!("{}", that);

    slice::basic_slice_stuff();
}

// data types with unknown size at compile time don't implement the "copy" trait
fn noncopy_ownership() -> String {
    let me = String::from("Hello World!");
    let mut you = me;
    // Below fails as me was moved to you, me can no longer be used
    //me.push_str(" How's tricks?");
    you.push_str(" My name is Billiam");

    // instead of moving the data and invalidating you all of the data is copied to a new variable
    // so you can use both of them independently
    let _him = you.clone();
    you.push_str(". Cabbage time;");

    you
}

// data types with known sizes implement copy, such as integers, floats, bools, chars and tuples thereof
fn copy_ownership() -> (i32, i32) {
    let mut x = 20;
    // as i32 implements copy, x doesn't move to y but rather the value is added to it
    // x and y are stored separately on the stack
    let y = x;
    // so you can modify x afterwards without affecting y
    x += 30;
    (x, y)
}

fn take_and_give(strin: String) -> String {
    strin
}

fn borrow_string(strong: &String) {
    println!("{}", strong.capacity());
    println!("{}", strong.len());
}

fn change_string(strang: &mut String) {
    strang.push_str(" Dikembe Mutombo taught me everything");
}
