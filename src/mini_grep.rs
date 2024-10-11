pub struct SearchConfig {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl SearchConfig {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // Check if env var exist, it's val not important
        let case_insensitive = std::env::var("CASE_INSENSITIVE").is_ok();
        // or
        // let case_insensitive = match std::env::var("CASE_INSENSITIVE"){
        //     Ok(_val) => true,
        //     Err(_) => false,
        // };

        Ok(Self {
            query,
            filename,
            case_insensitive,
        })
    }
}

// Search and PRINT the lines from `contents` that has str `query`
pub fn run_search(config: SearchConfig) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&config.filename)?;
    // or
    // .expect("Something went wrong reading the file");

    println!("\n{}:\n------\n{}\n", &config.filename, contents);

    print!("Searching for \"{}\" ", config.query);
    println!(
        "in file <{}> (case-insensitive:{}):\n------",
        config.filename, config.case_insensitive
    );

    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

// Return the lines from `contents` which has str `query`
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line)
        };
    }
    if res.is_empty() {
        res.push("No match found")
    }
    res
}

// or
pub fn __search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines() // make iter
        .filter(|line| line.contains(query)) // operation
        .collect() // make collection
}

// Return the lines from `contents` which has str `query`
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = vec![];
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line)
        };
    }
    if res.is_empty() {
        res.push("No match found")
    }
    res
}

#[cfg(test)]
mod str_search {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "Fast";
        let contents = "\
Rust:
safe, fast, productive.
Fast car";

        assert_eq!(vec!["Fast car"], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "fast";
        let contents = "\
Rust:
safe, fast, productive.
Fast car";

        assert_eq!(
            vec!["safe, fast, productive.", "Fast car"],
            search_case_insensitive(query, contents)
        );
    }
}
