#![feature(test, command_access)]

extern crate test;

use algorithms::find_shortest_path_length;
use graph::build_graph;
use std::path::PathBuf;
use test::Bencher;
use utils::PositiveInteger;

// Make sure to run `make benchmarks_data` before running benches

#[bench]
fn dijkstra_bench_1000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/1000_nodes.txt")).unwrap();
    b.iter(|| find_shortest_path_length(&graph.clone(), PositiveInteger::new(1), PositiveInteger::new(1000)));
}

#[bench]
fn dijkstra_bench_10000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/10000_nodes.txt")).unwrap();
    b.iter(|| find_shortest_path_length(&graph.clone(), PositiveInteger::new(1), PositiveInteger::new(10000)));
}

#[bench]
fn dijkstra_bench_100000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/100000_nodes.txt")).unwrap();
    b.iter(|| find_shortest_path_length(&graph.clone(), PositiveInteger::new(1), PositiveInteger::new(100000)));
}

#[bench]
fn dijkstra_bench_200000_nodes(b: &mut Bencher) {
    let graph = build_graph(&PathBuf::from("benches/data/200000_nodes.txt")).unwrap();
    b.iter(|| find_shortest_path_length(&graph.clone(), PositiveInteger::new(1), PositiveInteger::new(200000)));
}
