#![feature(test, command_access)]

extern crate test;

use algorithms::calculate_min_total_weight;
use graph::build_graph;
use std::path::PathBuf;
use test::Bencher;

// Make sure to run `make benchmarks_data` before running benches

#[bench]
fn bench_1000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/1000_nodes.txt")).unwrap();
    b.iter(|| calculate_min_total_weight(graph.clone()));
}

#[bench]
fn bench_10000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/10000_nodes.txt")).unwrap();
    b.iter(|| calculate_min_total_weight(graph.clone()));
}

#[bench]
fn bench_100000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/100000_nodes.txt")).unwrap();
    b.iter(|| calculate_min_total_weight(graph.clone()));
}

#[bench]
fn bench_200000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/200000_nodes.txt")).unwrap();
    b.iter(|| calculate_min_total_weight(graph.clone()));
}
