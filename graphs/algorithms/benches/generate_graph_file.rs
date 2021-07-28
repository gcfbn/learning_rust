#![feature(test)]

extern crate test;

use std::process::Command;
use test::Bencher;

fn generate_file_and_run_bench(nodes_count: i32) {
    // let makefile_string = format!("make generate_graph_{}_nodes", nodes_count);
    // let mut makefile_command = Command::new(makefile_string);

    let command_string = format!("make generate_graph_{}_nodes", nodes_count);

    let command = if cfg!(target_os = "windows") {
        Command::new("make").args(&["generate_graph_1000_nodes"]).output()
    } else {
        Command::new("make").args(&["generate_graph_1000_nodes"]).output()
    };

    // let mut command = Command::new("make");
    // let mut command = command.current_dir("/../../");
    // let mut command = command.arg("generate_graph_1000_nodes").spawn();

    // let mut command = Command::new("ls").spawn();

    let mut command = Command::new("makefile").args(&["generate_graph_1000_nodes"]).spawn();

    // let output = Command::new("ls")
    // .current_dir("/../../")
    // .arg("generate_graph_1000_nodes")
    // .output()
    // .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    // command.status().expect("NIET");

    println!("{:?}", command);

    assert!(command.is_ok());
}

#[bench]
fn bench_10000_nodes(b: &mut Bencher) {
    b.iter(|| generate_file_and_run_bench(10000))
}

#[bench]
fn bench_100000_nodes(b: &mut Bencher) {
    b.iter(|| generate_file_and_run_bench(100000))
}

#[bench]
fn bench_1000_nodes(b: &mut Bencher) {
    b.iter(|| generate_file_and_run_bench(1000))
}
