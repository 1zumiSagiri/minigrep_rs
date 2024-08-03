use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip the program name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: minigrep_rs <query> <file_path>"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Usage: minigrep_rs <query> <file_path>"),
        };

        let ignore_case = args.any(|arg| arg == "--ignore-case");

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // assign the contents or return Err
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let results: Vec<&str>;
    if ignore_case {
        results = contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
            .collect();
    } else {
        results = contents
            .lines()
            .filter(|line| line.contains(&query))
            .collect();
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_true_result() {
        let query = "Duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        // case-insensitive search
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn one_false_result() {
        let query = "Duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        // case-sensitive search
        // need type hint for Vec<&str> to avoid ambiguity
        assert_eq!(Vec::<&str>::new(), search(query, contents, false));
    }
}
