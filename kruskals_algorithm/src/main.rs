#[macro_use]
extern crate clap;

use std::process::exit;
use std::path::Path;

fn main() {
    // generate menu using Clap
    let matches = clap_app!(kruskal_algorithm =>
        (version: "1.0")
        (author: "Bartek M. <bmekarski@interia.pl>")
        (about: "Algorithms & Data structures task from graph theory")
        (@arg TASK: -t --task "Prints info about task")
        (@arg RUN: -r --run +takes_value "Runs the program")
        )
        .get_matches();

    if let Some(file_name) = matches.value_of("RUN") {
        let result = kruskals_algorithm::run(Path::new(file_name)).unwrap_or_else(|error| {
            eprintln!("An error happened: {}", error);
            exit(1);
        });
        println!("{}", result);
    }

    if matches.occurrences_of("TASK") > 0 {
        println!("Task is available in task.txt");
    }
}