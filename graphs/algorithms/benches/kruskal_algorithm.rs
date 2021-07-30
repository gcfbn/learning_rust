#![feature(test, command_access)]

extern crate test;

use algorithms::calculate_min_total_weight;
use graph::{build_graph, Graph};
use std::path::{Path, PathBuf};
use std::process::Command;
use test::Bencher;

// before each bench
fn generate_file_and_build_graph(nodes_count: i32) -> Graph {
    let make_command = format!("make generate_graph_{}_nodes", nodes_count);

    let command = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &make_command])
            .current_dir("./../")
            .output()
    } else {
        Command::new("sh")
            .args(&["-c", &make_command])
            .current_dir("./../")
            .output()
    };

    assert!(command.is_ok());

    let graph_path = PathBuf::from(format!("benches/data/{}_nodes.txt", nodes_count));
    build_graph(&graph_path).unwrap()
}

#[bench]
fn bench_1000_nodes(b: &mut Bencher) {
    let graph = generate_file_and_build_graph(1000);
    b.iter(|| calculate_min_total_weight(graph.clone()));
}

#[bench]
fn bench_10000_nodes(b: &mut Bencher) {
    let graph = generate_file_and_build_graph(10000);
    b.iter(|| calculate_min_total_weight(graph.clone()));
}

// causes stack overflow
// #[bench]
// fn bench_100000_nodes(b: &mut Bencher) {
//     let graph = generate_file_and_build_graph(100000);
//     b.iter(|| calculate_min_total_weight(graph.clone()));
// }
