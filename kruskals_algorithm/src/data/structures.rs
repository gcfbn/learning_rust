use anyhow::Error;

#[derive(Copy, Clone, Debug)]
pub struct Edge {
    pub from_index: u32,
    pub to_index: u32,
    pub weight: i32,
}

impl Edge {
    pub fn try_from_str(from_index: &str, to_index: &str, weight: &str) -> Result<Edge, Error> {
        let parsed_from_index = from_index.parse::<u32>()?;
        let parsed_to_index = to_index.parse::<u32>()?;
        let parsed_weight = weight.parse::<i32>()?;

        Ok(Edge::new(parsed_from_index, parsed_to_index, parsed_weight))
    }

    pub fn new(from_index: u32, to_index: u32, weight: i32) -> Edge {
        Edge {
            from_index,
            to_index,
            weight,
        }
    }
}

pub struct Graph {
    pub nodes_count: u32,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes_count: u32, edges: Vec<Edge>) -> Graph {
        Graph {
            nodes_count,
            edges,
        }
    }
}