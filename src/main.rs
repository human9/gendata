extern crate gendata;
use gendata::*;

use std::env;
use std::process;
use std::error::Error;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|e| {
        println!("Error: {}", e);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("Error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    
    match config.operation.as_ref() {
        "gen" => gen_data(&config.remainder[0], &config.remainder[1])?,
        _ => println!("Unimplemented operation"),
    }

    Ok(())
}

struct Config {
    operation: String,
    remainder: Vec<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        args.next(); // consume the first argument

        let operation = match args.next() {
            Some(arg) => arg,
            None => return Err("No operation specified".to_string()),
        };

        let remainder: Vec<String> = args.collect();

        match operation.as_ref() {
            "gen" => { if remainder.len() < 2 { return Err(format!("{}: too few inputs specified", operation)) } },
            _ => return Err(format!("{}: Unknown operation", operation)),
        }
            

        Ok(Config { operation, remainder })
    }
}
