use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read the file with error: {}", err);
        process::exit(1);
    });

    let result = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(t) => t,
            None => return Err("Didnt get the query string"),
        };

        let file_path = match args.next() {
            Some(t) => t,
            None => return Err("Didnt get the file path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(
            vec!["        safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["Rust:"], search_insensitive(query, contents));
    }
}
