use std::process;

fn main() {
    let result = kruskals_algorithm::run("input.txt").unwrap_or_else(|error| {
        eprintln!("An error happened: {}", error);
        process::exit(1);
    });
    println!("{}", result);
}