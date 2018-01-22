use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        //By replacing what was before a vector of stings with an itertor
        //we can now iterate through the env args. This also let's us
        //provide more specific error messages than last time.
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        //This is how we can check an environmental variable in
        //rust
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    //&str has a lines method that breaks the contents
    //of a string up by the lines and lets us iterate over 
    //those
    // for line in contents.lines(){
    //     //&str also has a method that allows us to check
    //     //if the string contains another string
    //     if line.contains(query){
    //         //If it does contain the string we
    //         //push it that line to our results
    //         results.push(line);
    //     }
    // }
    // results
//We can replace all of the above by using an iterator and the filter
//method.
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()

}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //First transform query to lowercase
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        //Next transform line to lowercase and check to see if
        //query is contained in line
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    
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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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