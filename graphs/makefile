.PHONY: doc

doc:
	cargo doc -p graph -p runner -p algorithms --document-private-items

generate_graph_1000_nodes:
	cargo run -- gfg algorithms/tests/benchmarks_data/1000_nodes.txt 1000 3000 100

generate_graph_10000_nodes:
	cargo run -- gfg algorithms/tests/benchmarks_data/10000_nodes.txt 10000 30000 100

generate_graph_100000_nodes:
	cargo run -- gfg algorithms/tests/benchmarks_data/100000_nodes.txt 100000 300000 100