extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //env::args() returns an iterator of the command line arguments given to the program
    //This will panic if the strings inputted aren't valid UTF-8.
    //If you need inputs that aren't valid use env::args_os which returns OsString
    //values.
    //The collect function can create several different collections.
    //Therefore, you need to make sure you annotate the type your expecting.
    let args: Vec<String> = env::args().collect();
    //The code after unwrap_or_else is a closure
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //Moved all of the logic into a run function
    //We know let the user know if there was an error in the main application
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

