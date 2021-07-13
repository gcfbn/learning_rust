.PHONY: doc

doc:
	cargo doc -p graph -p runner -p algorithms --document-private-items

benchmarks_data: generate_graph_1000_nodes generate_graph_10000_nodes generate_graph_100000_nodes

generate_graph_1000_nodes: algorithms/benches/data/1000_nodes.txt
algorithms/benches/data/1000_nodes.txt:
	cargo run -- gfg --graph-file algorithms/benches/data/1000_nodes.txt --nodes-count 1000 --edges-count 3000 --max-weight 100

generate_graph_10000_nodes: algorithms/benches/data/10000_nodes.txt
algorithms/benches/data/10000_nodes.txt:
	cargo run -- gfg --graph-file algorithms/benches/data/10000_nodes.txt --nodes-count 10000 --edges-count 30000 --max-weight 100

generate_graph_100000_nodes: algorithms/benches/data/100000_nodes.txt
algorithms/benches/data/100000_nodes.txt:
	cargo run -- gfg --graph-file algorithms/benches/data/100000_nodes.txt --nodes-count 100000 --edges-count 300000 --max-weight 100
