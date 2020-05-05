use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    // the panic macro stops and unwinds the program
    // panic is called when the compiler encounters an unhandled error

    match get_result(52) {
        Ok(_) => println!("You win!"),
        Err(a) => println!("{:?}", a),
    }

    match File::open("dontexist.txt") {
        Ok(a) => {
            println!("{:?}", a);
            // make a hashmap that immediately drops out of scope for no good reason
            let mut hoshmop = HashMap::new();
            hoshmop.insert(String::from("Confucius"), String::from("?"));
        },
        Err(e) => panic!("{:?}", e),
    }

    // Can't use ? operator as main doesn't return result, so use expect instead (could also use match)
    let me = get_result(7).expect("Something's gone wrong");

    panic!("Unrecoverable, big sad");
}

// bubble errors up so that all errors are handled by one error handling section
fn get_result(num: i32) -> Result<bool, String> {
    if num < 1 {
        Ok(false)
    } else {
        Err(String::from("You gave a bad number"))
    }
}
