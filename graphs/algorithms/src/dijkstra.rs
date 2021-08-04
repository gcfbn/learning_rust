use graph::{adjacency_list, Graph};
use std::cmp::{Ordering, PartialOrd};
use std::collections::BinaryHeap;
use thiserror::Error;
use utils::PositiveInteger;

pub type Result<T> = std::result::Result<T, DijkstraAlgorithmError>;

#[derive(PartialEq, Eq)]
struct NodeDistance {
    index:    u32,
    distance: u32,
}

impl NodeDistance {
    pub fn new(index: u32, distance: u32) -> NodeDistance {
        NodeDistance { index, distance }
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Error, Debug)]
pub enum DijkstraAlgorithmError {
    #[error("start_node `{start_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidStartNode { start_node: u32, nodes_count: u32 },

    #[error("end_node `{end_node}` is greater than nodes_count `{nodes_count}`")]
    InvalidEndNode { end_node: u32, nodes_count: u32 },
}

fn is_node_index_valid(index: u32, nodes_count: u32) -> bool {
    index <= nodes_count
}

fn validate_nodes(start_node: u32, end_node: u32, nodes_count: u32) -> Result<()> {
    if !is_node_index_valid(start_node, nodes_count) {
        return Err(DijkstraAlgorithmError::InvalidStartNode {
            start_node,
            nodes_count,
        });
    }
    if !is_node_index_valid(end_node, nodes_count) {
        return Err(DijkstraAlgorithmError::InvalidEndNode { end_node, nodes_count });
    }

    Ok(())
}

pub fn find_shortest_path_length(graph: &Graph, start_node: PositiveInteger, end_node: PositiveInteger) -> Result<u32> {
    validate_nodes(start_node.value(), end_node.value(), graph.nodes_count)?;

    // create adjacency list describing given graph
    let adjacency_list = adjacency_list(&graph.edges, graph.nodes_count);

    // create empty binary heap
    let mut heap = BinaryHeap::new();

    // create vector of already calculated distances (initialized with usize::MAX)
    // nodes are indexed from 1, so this vec has length adjacency_list.len() + 1
    let mut calculated_distances: Vec<u32> = (0..=adjacency_list.len()).map(|_| u32::MAX).collect();

    // starting node has distance == 0
    calculated_distances[start_node.value() as usize] = 0;
    heap.push(NodeDistance::new(start_node.value(), 0));

    while let Some(popped_node) = heap.pop() {
        // if popped node is the one we are looking for
        if end_node == popped_node.index {
            return Ok(popped_node.distance);
        }

        // if we already found a better way to popped node
        if popped_node.distance > calculated_distances[popped_node.index as usize] {
            continue;
        }

        // for every neighbour of popped_node see if we can find a shorter way
        for (target_node, weight) in &adjacency_list[(popped_node.index as usize)] {
            let new_distance = popped_node.distance + *weight as u32;

            if new_distance < calculated_distances[*target_node] {
                heap.push(NodeDistance::new(*target_node as u32, new_distance));
                calculated_distances[*target_node] = new_distance;
            }
        }
    }

    panic!("This shouldn't happen, because graph is connected");
}
