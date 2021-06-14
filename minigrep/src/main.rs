use std::process;
use std::env;


use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", cfg.query);
    println!("In file {}", cfg.filename);

    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
