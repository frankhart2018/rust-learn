use std::error::Error;
use std::fs;
use std::env;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // is_err() is used to unset case_senstivie, which means searching will be done
        // in a case sensitive manner
        // is_err() will be called when the env::var() method fails to find the environment variable
        // We don't care about the value of the environment variable, we just care
        // if it is set or unset
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query, 
            filename,
            case_sensitive,
        })
    }
}

// Box<dyn Error> that the function will retur a type that implements the Error trait
// but we don't have to specify what particular type the return value will be
// This gives us flexibility to return error values that may be different types in different erro cases
// dyn is short for 'dynamic'
pub fn run(config: Config) -> Result<(), Box<dyn Error>> { 
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// By adding lifetime parameter 'a we specify that the returning vector
// will contain string slices from contents, since query won't be part of that
// so that is not marked with this lifetime parameter
// This is important as the data referenced in the vector needs to be a valid reference
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase(); // to_lowercase() converts &str to String
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) { // contains() expects a string slice as arg, so & is added
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn case_sensitive() {
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
Picke three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
