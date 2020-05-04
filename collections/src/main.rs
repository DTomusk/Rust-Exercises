mod strongs;
mod hush;
// collections contain multiple values
// collection data is stored on the heap, so the compiler doesn't need to know the size before runtime
// collections can grow or shrink during run time

// making a vector of an enum will allow you to store many different data types in one vector
enum DifferentTypes {
    Words(String),
    Blips(bool),
    Blops(f64),
    Mordecai(Vec<u8>),
}

fn main() {
    // instantiating a vector, if it's empty you have to specify the type its values are gonna take
    let mut vev: Vec<bool> = Vec::new();
    // similar to instantiating an array, you just prepend the vec! macro
    let mut viv = vec![1.2, 3.2, 7.0];
    vev.push(false);
    vev.push(true);
    viv.push(12.07);

    // when a vector is dropped so is all of its contents
    {
        let vav = vec!['s'];
        // vector goes out of scope here
    }

    // immutable reference to vector, similar again to arrays
    println!("{:?}", &vev[0]);

    // vector.get(index) gets the value in vector at index index, if there's none it returns None
    match viv.get(1) {
        Some(melon) => println!("The second element of viv is: {:?}", melon),
        None => println!("viv has no second element"),
    }

    // iterates over all the values in viv using a mutable reference
    for v in &mut viv {
        // dereference value to make change
        *v -= 2.2;
    }

    let mut vov: Vec<DifferentTypes> = Vec::new();
    vov.push(DifferentTypes::Words(String::from("Hello there")));
    vov.push(DifferentTypes::Mordecai(vec![2,4,6,87]));

    let kaiser = viv.pop();

    strongs::string_stuff();
}
