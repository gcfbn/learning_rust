// extern these crates only when running tests
#[cfg(test)]
extern crate test_case;

mod algorithm;

pub use algorithm::calculate_min_total_weight;
