fn main() {
    let this = noncopy_ownership();
    let (x, y) = copy_ownership();
    println!("x: {}", x);
    println!("y: {}", y);

    let that = take_and_give(this);

    println!("{:?}", that);

    // everything drops out of scope
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
