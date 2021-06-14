use super::structures;
use std::fs;
use std::error::Error;

pub fn build_graph_from_input(filename: &str) -> Result<structures::Graph, Box<dyn Error>> {
    // println!("In file {}", filename);

    let input = fs::read_to_string(filename)?;

    // println!("{}", input);

    let splitted_input = input.split_whitespace().map(|s| s.to_string()).collect();

    let parameters = Parameters::new(&splitted_input)?;

    // input should contain n, m and 3 numbers for every of m lines
    if splitted_input.len() as u32 != parameters.m * 3 + 2 {
        return Err(Box::from("Input does not match declared number of edges"));
    }

    let mut edges: Vec<structures::Edge> = Vec::new();

    for i in (2..2 + parameters.m * 3).step_by(3) {
        edges.push(structures::Edge::new(
            &splitted_input[i as usize],
            &splitted_input[(i + 1) as usize],
            &splitted_input[(i + 2) as usize])?);
    }

    Ok(structures::Graph::new(parameters.n, edges))
}

pub struct Parameters {
    n: u32,
    m: u32,
}

impl Parameters {
    fn new(input: &Vec<String>) -> Result<Parameters, Box<dyn Error>> {
        // i am not sure how it works, but it works
        // probably, that's how error boxing should be used
        // Box::from(err: &str) returns Box<dyn Error> and that's what I need here
        if input.len() < 2 { return Err(Box::from("Not enough data in input")); }

        // if let Ok(n) = input.get(0).unwrap().parse::<i32>() {} else {
        //     return Err(error);
        // }

        let n: u32 = input[0].parse::<u32>()?;
        let m: u32 = input[1].parse::<u32>()?;

        Ok(Parameters {
            n,
            m,
        })
    }
}