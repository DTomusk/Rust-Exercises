/*
String is growable, mutable, owned, UTF-8 encoded string type
&str is a slice of a String
Strings are owned &str's are borrowed
*/

pub fn string_stuff() {
    let mut strang = String::new();
    // instantiate string literal (hardcoded characters)
    let words = "Difficult magik";
    // change string literal to String
    let mut mords = words.to_string();
    mords = "Melancholy melon collie".to_string();
    let bords = String::from("Borga borga borga boo");

    mords.push_str(" barabarabas");
    let mut strung = String::from("foo");
    let streng = " fighters";
    strung.push_str(streng);
    // push a single character
    strung.push(' ');

    let strong = mords + &strung;
    let beth = format!("{} {}", strong, streng);

    // a string is a wrapper over a Vec<u8>
    // strings are more complicated than other languages make them out to be
    // you can interate over a string by converting it into an array of characters
    for c in beth.chars() {
        println!("{:?}", c);
    }
}
