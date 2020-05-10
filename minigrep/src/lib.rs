// functions involving the file system, reads, writes files and so on
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // clone usually isn't the optimal solution but its impact here is minimal
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// decouple the logic from main
// Box<dyn Error> is a trait object that returns a type that implements the Error trait
// we don't have to specify what type the error object will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // creates a string from the contents of the file at the path filename
    let contents = fs::read_to_string(config.filename)?;

    // run doesn't return a value we need
    Ok(())
}
