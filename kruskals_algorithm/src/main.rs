use std::process;

// TODO use clippy and check warnings

fn main() {
    let result = kruskals_algorithm::run("input.txt").unwrap_or_else(|error| {
        eprintln!("An error happened: {}", error);
        process::exit(1);
    });

    println!("nodes_count: {}", result.nodes_count);

    for e in result.edges {
        println!("{:?}", e);
    }
}