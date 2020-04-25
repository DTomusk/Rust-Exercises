pub fn basic_slice_stuff() {
    let strung = String::from("Eminem's discography");
    // mother and father are slices of strung
    // slices are references to part of a string
    let mother = &strung[2..8];
    let _father = &strung[1..];
    // gentleman and lady are equivalent
    let _gentleman = &strung[0..strung.len()];
    let _lady = &strung[..];

    println!("{}", mother);

    let organa = get_first(&strung);
    println!("{}", organa);

    // the type of a string literal is &str, it's an immutable reference to a hardcoded point in the binary
    let _medic = "Orthopoetics";

    // in addition to string slices you can have array slices
    let arrr = [1, 2, 7, 3];
    println!("{:?}", arrr);
    // this slice has type &[i32]
    println!("{:?}", &arrr[..2]);
}

// this is essentially the same as the code in the book but I thought it would be interesting to look at
// &str is the signifies a string slice
// here the value is tied to the underlying data, so it changes if the string changes
fn get_first(streng: &str) -> &str {
    // haven't looked at iterators yet
    // takes the string, treats it as bytes, iterates over it and enumerates to get i
    for (i, &item) in streng.as_bytes().iter().enumerate() {
        // if the byte is a space then we've found the end of the first word
        if item == b' ' {
            return &streng[..i];
        }
    }

    // if there is no space then evidently the whole string is one word so return a slice of the entire thing
    &streng[..]
}
