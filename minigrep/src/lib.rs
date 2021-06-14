use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("Containing:\n{}", contents);

    Ok(())
}

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}