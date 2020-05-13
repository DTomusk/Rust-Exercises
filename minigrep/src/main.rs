// this will essentially be a copy of the chapter 12 project in the rust book with my own annotations

// contains funcions pertaining to the current environment, used to get command line arguments, directory paths etc.
// while we only use env::args, it's convention to bring the parent module into scope
use std::env;
// will be used for exiting the program
use std::process;
// need to call config struct into scope to use it
use minigrep::Config;

// main parses the arguments and calls the logic to run the program with them
fn main() {
    // not sure what a closure is yet, will find out later
    // unwrap or else gets the value from the result if it's Ok or else handles the error case
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln prints to stderr instead of stdout
        eprintln!("Problem parsing arguments: {}", err);
        // exiting with a non zero code is a standard way of showing an error has occurred
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
