use graph::Graph;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct UnionFind {
    parent: Vec<u32>,
    rank:   Vec<u8>,
}

impl UnionFind {
    /// n - count of disjoint sets.
    pub fn new(n: u32) -> Self {
        let rank = vec![0; (n + 1) as usize];
        let mut parent = Vec::with_capacity(n as usize);

        for i in 0..=n {
            parent.push(i);
        }

        UnionFind { parent, rank }
    }

    /// find parent of element with given index
    pub fn find_parent(&self, index: u32) -> u32 {
        assert!(index < self.parent.len() as u32);

        let mut current_index = index;
        while self.parent[current_index as usize] != current_index {
            current_index = self.parent[current_index as usize];
        }
        current_index
    }

    /// `find_parent` with path compression
    pub fn find_mut_parent(&mut self, index: u32) -> u32 {
        assert!(index < self.parent.len() as u32);

        let mut index = index;

        let mut parent = self.parent[index as usize];
        while parent != index {
            let grandparent = self.parent[parent as usize];
            self.parent[index as usize] = grandparent;
            index = parent;
            parent = grandparent;
        }
        index
    }

    /// call `union` method for parents of given elements
    pub fn merge_parents(&mut self, x: u32, y: u32) -> bool {
        let x_parent = self.find_parent(x);
        let y_parent = self.find_parent(y);
        if x_parent != y_parent {
            self.union(x_parent, y_parent)
        } else {
            false
        }
    }

    /// merge two sets
    pub fn union(&mut self, x: u32, y: u32) -> bool {
        if x == y {
            return false;
        }
        let xrep = self.find_mut_parent(x);
        let yrep = self.find_mut_parent(y);

        if xrep == yrep {
            return false;
        }

        let xrepu = xrep as usize;
        let yrepu = yrep as usize;

        let xrank = self.rank[xrepu];
        let yrank = self.rank[yrepu];

        match xrank.cmp(&yrank) {
            Ordering::Less => self.parent[xrepu] = yrep,
            Ordering::Greater => self.parent[yrepu] = xrep,
            Ordering::Equal => {
                self.parent[yrepu] = xrep;
                self.rank[xrepu] += 1;
            }
        }

        true
    }
}

/// Uses Kruskal's algorithm to calculate weight of graph minimum spanning tree
///
/// # Arguments
///
/// * 'graph' - connected graph that will be used to calculate weight of minimum spanning tree
pub fn calculate_min_total_weight(mut graph: Graph) -> i32 {
    let mut union_find = UnionFind::new(graph.nodes_count);

    graph.edges.sort_by(|a, b| a.weight.cmp(&b.weight));

    graph
        .edges
        .iter()
        .filter(|e| union_find.merge_parents(e.from_index, e.to_index))
        .map(|e| e.weight as i32)
        .sum()
}
