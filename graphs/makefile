.PHONY: doc

doc:
	cargo doc -p graph -p runner -p algorithms --document-private-items

benchmarks_data: generate_graph_1000_nodes generate_graph_10000_nodes generate_graph_100000_nodes

generate_graph_1000_nodes: 1000_nodes.txt.generated

1000_nodes.txt.generated:
	cargo run -- gfg algorithms/tests/benchmarks_data/1000_nodes.txt 1000 3000 100

generate_graph_10000_nodes: 10000_nodes.txt.generated

10000_nodes.txt.generated:
	cargo run -- gfg algorithms/tests/benchmarks_data/10000_nodes.txt 10000 30000 100

generate_graph_100000_nodes: 100000_nodes.txt.generated

100000_nodes.txt.generated:
	cargo run -- gfg algorithms/tests/benchmarks_data/100000_nodes.txt 100000 300000 100