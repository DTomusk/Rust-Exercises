// functions involving the file system, reads, writes files and so on
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query input")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename input"),
        };

        let case_sensitive = match args.next() {
            Some(arg) => {
                if arg == "-i" {
                    false
                } else {
                    return Err("Invalid option input")
                }
            },
            None => true,
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// decouple the logic from main
// Box<dyn Error> is a trait object that returns a type that implements the Error trait
// we don't have to specify what type the error object will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // creates a string from the contents of the file at the path filename
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // run doesn't return a value we need
    Ok(())
}

// the explicit lifetime shows that the return value's lifetime comes from the contents argument
// we're saying the returned vector contains references to contents rather than query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // turns contents into an iterator of lines, filters them based on whether they contain the desired string
    //then collects them into a vector with the desired signature 
    contents.lines().filter(|line| line.contains(query)).collect()
}

// I'm not sure this function is completely necessary as the logic is very similar to search
// could refactor and add an option to search instead of having two functions doing similar things
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/* test driven development:
write a test to check the desired functionality
modify the code the minimum amount to get the test to pass
refactor the new code while ensuring the test still passes
start the process all over again
*/
#[cfg(test)]
mod tests {
    use super ::*;

    #[test]
    fn case_sensitive() {
        // search for the string "duct" in the given contents
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
