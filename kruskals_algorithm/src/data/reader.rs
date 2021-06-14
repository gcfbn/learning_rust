use super::structures;
use std::fs;
use std::error::Error;

pub fn read_graph_data(filename: &str) -> Result<structures::Graph, Box<dyn Error>> {
    // println!("In file {}", filename);

    let input = fs::read_to_string(filename)?;

    // println!("{}", input);

    let splitted_input = input.split_whitespace().map(|s| s.to_string()).collect();

    let parameters = Parameters::new(&splitted_input)?;
    let mut edges: Vec<structures::Edge> = Vec::new();

    let mut index = 2;
    let mut edge: structures::Edge;

    for i in 0..parameters.m {
        edge = structures::Edge::new(
            // TODO check if index is in bounds
            &splitted_input[index],
            &splitted_input[index + 1],
            &splitted_input[index + 2])?;

        edges.push(edge.clone());
        index = index + 3;
    }

    Ok(structures::Graph::new(parameters.n, edges))
}

pub struct Parameters {
    n: i32,
    m: i32,
}

impl Parameters {
    fn new<'a>(input: &'a Vec<String>) -> Result<Parameters, Box<dyn Error>> {
        // i am not sure how it works, but it works
        // probably, that's how error boxing should be used
        // Box::from(err: &str) returns Box<dyn Error> and that's what I need here
        if input.len() < 2 { return Err(Box::from("Not enough data in input")); }

        // if let Ok(n) = input.get(0).unwrap().parse::<i32>() {} else {
        //     return Err(error);
        // }

        let n = input[0].parse::<i32>()?;
        let m = input[1].parse::<i32>()?;

        Ok(Parameters {
            n,
            m,
        })
    }
}