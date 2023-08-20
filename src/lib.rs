use std::env;
use std::fs;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Expected 2 args: <query> <file_path>\n");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        return Ok(Config { query, file_path, ignore_case });
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    
    println!("Contents: \n{}", contents);

    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    println!("Result: ");
    for line in results {
        println!("{line}");
    }
    println!();

    return Ok(());
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}


pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}


#[cfg(test)]
mod minigrep {
    use super::*;

    #[test]
    fn case_sensitive() {
        // Read file contents
        let file_path = "./src/book/chapter_12/preface.txt";
        let contents = fs::read_to_string(&file_path)
            .expect(&format!("Reading file: {}...", &file_path));
        
        // Define query and expected result
        let query = "it";
        let expected_result = vec![
            "relativity, quantum theory was not created—or even definitively",
            "packaged—by one individual, and it retains to this day some of the",
            "scars of its exhilirating but traumatic youth. There is no general",
            "consensus as to what its fundamental principles are, how it should be",
            "taught, or what it really \"means\". Every competent physicist can \"do\"",
            "implausible. Richard Feynman (one of the greatest practitioners)",
        ];

        // Assert equality
        assert_eq!(
            expected_result,
            search(query, &contents)
        );
    }

    #[test]
    fn case_insensitive() {
        // Read file contents
        let file_path = "./src/book/chapter_12/preface.txt";
        let contents = fs::read_to_string(&file_path)
            .expect(&format!("Reading file: {}...", &file_path));

        // Define query and expected result
        let query = "iT";
        let expected_result = vec![
            "relativity, quantum theory was not created—or even definitively",
            "packaged—by one individual, and it retains to this day some of the",
            "scars of its exhilirating but traumatic youth. There is no general",
            "consensus as to what its fundamental principles are, how it should be",
            "taught, or what it really \"means\". Every competent physicist can \"do\"",
            "implausible. Richard Feynman (one of the greatest practitioners)",
        ];

        // Assert equality
        assert_eq!(
            expected_result,
            search_insensitive(query, &contents)
        );
    }
}
