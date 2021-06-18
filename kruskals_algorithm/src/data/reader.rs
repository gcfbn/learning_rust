use super::structures;
use std::fs;
use anyhow::{bail, Error};
use std::path::Path;
use crate::data::structures::Edge;

pub fn build_graph_from_input(filename: &Path) -> Result<structures::Graph, Error> {
    // println!("In file {}", filename);

    // let input = fs::read_to_string(filename)?;

    // println!("{}", input);

    // let splitted_input = input.split_whitespace().map(|s| s.to_string()).collect();

    let mut parameters = Parameters::try_from_path(filename)?;

    let triple_iterator = IteratorOverThree::new(&mut parameters.iter);
    let mut edges: Vec<structures::Edge> = Vec::new();

    let mut current_edge: Edge;

    for (from_index, to_index, weight) in triple_iterator {
        current_edge = Edge::try_from_str(from_index, to_index, weight)?;
        edges.push(current_edge);
    }

    // // input should contain n, m and 3 numbers for every of m lines
    // if splitted_input.len() as u32 != parameters.m * 3 + 2 {
    //     return Err(anyhow!("Wrong input length, contains {} elements, should be {}",
    //     splitted_input.len(), parameters.m * 3 + 2));
    // }
    //
    // let mut edges: Vec<structures::Edge> = Vec::new();
    //
    // for i in (2..2 + parameters.m * 3).step_by(3) {
    //     edges.push(structures::Edge::new(
    //         &splitted_input[i as usize],
    //         &splitted_input[(i + 1) as usize],
    //         &splitted_input[(i + 2) as usize])?);
    // }

    Ok(structures::Graph::new(parameters.n, edges))
}

pub struct Parameters<'a> {
    n: u32,
    m: u32,
    iter: &'a mut (dyn Iterator<Item=&'a str> + 'a),
}

impl<'a> Parameters<'a> {
    fn new(n: u32, m: u32, mut iter: impl Iterator<Item=&'a str> + 'a) -> Parameters<'a> {
        Parameters {
            n,
            m,
            iter: &mut iter,
        }
    }

    fn try_from_input(input: &'a String) -> Result<Parameters<'a>, Error> {
        let mut input_iter = input.split_whitespace();
        // let n = match input_iter.next().parse::<u32>() {
        //     Ok(value) => value,
        //     Err(_) => anyhow!("n is not a positive integer!"),
        // };
        // let m = match input_iter.next().parse::<u32>() {
        //     Ok(value) => value,
        //     Err(_) => anyhow!("m is not a positive integer"),
        // };

        // let n = input_iter.next()?.parse::<u32>()?;
        // let m = input_iter.next()?.parse::<u32>()?;

        let n = input_iter.next();
        let m = input_iter.next();

        let n = match n {
            Some(value) => value,
            None => bail!("Not enough data!"),
        };

        let m = match m {
            Some(value) => value,
            None => bail!("Not enough data!"),
        };

        let n = n.parse::<u32>()?;
        let m = m.parse::<u32>()?;

        Ok(Parameters::new(n, m, input_iter))
    }

    fn try_from_path(path: &Path) -> Result<Parameters, Error> {
        let input = fs::read_to_string(path)?;
        Parameters::try_from_input(&input)
    }
}

struct IteratorOverThree<'a> {
    inner_iterator: &'a mut (dyn Iterator<Item=&'a str> + 'a),
}

impl<'a> IteratorOverThree<'a> {
    fn new(iterator: &'a mut (impl Iterator<Item=&'a str> + 'a)) -> IteratorOverThree {
        IteratorOverThree { inner_iterator: iterator }
    }
}

impl<'a> Iterator for IteratorOverThree<'a> {
    type Item = (&'a str, &'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.inner_iterator.next()?;
        let second = self.inner_iterator.next()?;
        let third = self.inner_iterator.next()?;
        Some((first, second, third))
    }
}
