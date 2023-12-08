use itertools::Itertools;
use num::integer::lcm;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);

    let current_nodes = input
        .nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.to_string())
        .collect_vec();

    let end_nodes = input
        .nodes
        .keys()
        .filter(|n| n.ends_with("Z"))
        .map(|n| n.as_str())
        .collect_vec();

    let minimum_steps = current_nodes
        .iter()
        .map(|n| count_steps_to_end(&input, n, end_nodes.clone()))
        .collect_vec();

    let mut value = minimum_steps[0].clone() as u64;

    for num_instructions in minimum_steps.into_iter().skip(1) {
        value = lcm(value, num_instructions as u64);
    }

    if use_sample {
        assert_eq!(value, 6);
    } else {
        println!("{:?}", value);
    }
}
