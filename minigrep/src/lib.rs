// functions involving the file system, reads, writes files and so on
use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // clone usually isn't the optimal solution but its impact here is minimal
        let query = args[1].clone();
        let filename = args[2].clone();

        let mut case_sensitive = true;

        // option for case insensitive
        if args.len() == 4 {
            if args[3] == "-i" || args[3] == "--insensitive" {
                case_sensitive = false
            } else {
                return Err("Unrecognised option");
            }
        }

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
    let mut results = Vec::new();
    // .lines() returns an iterator which separates a str into lines
    for line in contents.lines() {
        // .contains() checks whether a certain string is contained in another string
        if line.contains(query) {
            results.push(line);
        }
    }
    results
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
