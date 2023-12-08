use itertools::Itertools;
use num::integer::lcm;
use std::collections::HashMap;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);

    let mut steps = 0u32;

    let mut current_nodes = input
        .nodes
        .keys()
        .filter(|n| n.ends_with("A"))
        .map(|n| n.to_string())
        .collect_vec();

    let mut min_num_instructions_map: HashMap<u8, u32> = HashMap::new();

    loop {
        for index in 0..input.instructions.len() {
            let instruction = input.instructions.get(index).unwrap();

            current_nodes = current_nodes
                .iter()
                .map(|n| {
                    let node: &NodeInstructions = input.nodes.get(n).unwrap();

                    match instruction {
                        Instruction::Left => node.left.to_string(),
                        Instruction::Right => node.right.to_string(),
                    }
                })
                .collect_vec();

            steps += 1;

            let matches = current_nodes.iter().map(|n| n.ends_with("Z")).collect_vec();
            for (index, is_match) in matches.iter().enumerate() {
                if *is_match {
                    let index = index as u8;
                    min_num_instructions_map.entry(index).or_insert(steps);
                }
            }

            if current_nodes.iter().all(|n| n.ends_with("Z")) {
                break;
            }
        }

        if min_num_instructions_map.len() == current_nodes.len() {
            break;
        }

        if current_nodes.iter().all(|n| n.ends_with("Z")) {
            break;
        }
    }

    let min_num_instructions = min_num_instructions_map.into_values().collect_vec();

    let mut value = min_num_instructions[0].clone() as u64;

    for num_instructions in min_num_instructions.into_iter().skip(1) {
        value = lcm(value, num_instructions as u64);
    }

    if use_sample {
        assert_eq!(value, 6);
    } else {
        println!("{:?}", value);
    }
}
