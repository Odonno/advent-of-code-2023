use itertools::Itertools;
use std::collections::HashSet;

use super::common::*;

#[derive(Debug, Clone, Copy)]
struct Gear {
    part_number_1: EnginePartNumber,
    part_number_2: EnginePartNumber,
}

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);
    let map = fill_map_with_part_numbers_on_empty_positions(&map);

    let possible_gear_positions = map
        .clone()
        .into_iter()
        .filter_map(|(pos, part)| match part {
            EnginePart::Symbol('*') => Some(pos),
            _ => None,
        })
        .collect_vec();

    let gears = possible_gear_positions
        .into_iter()
        .filter_map(|(x, y)| {
            let adjacent_positions = (x.saturating_sub(1)..=x + 1)
                .flat_map(|adj_x| {
                    ((y.saturating_sub(1))..=(y + 1)).map(move |adj_y| (adj_x, adj_y))
                })
                .filter(|(adj_x, adj_y)| adj_x != &x || adj_y != &y);

            let adjacent_part_numbers = adjacent_positions
                .filter_map(|pos| map.get(&pos))
                .filter_map(|part| match part {
                    EnginePart::Number(part) => Some(part),
                    _ => None,
                })
                .collect::<HashSet<_>>()
                .into_iter()
                .collect_vec();

            if adjacent_part_numbers.len() == 2 {
                let part_number_1 = adjacent_part_numbers[0].clone();
                let part_number_2 = adjacent_part_numbers[1].clone();

                Some(Gear {
                    part_number_1,
                    part_number_2,
                })
            } else {
                None
            }
        })
        .collect_vec();

    let value = gears
        .iter()
        .map(|gear| gear.part_number_1.value * gear.part_number_2.value)
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 467835);
    } else {
        println!("{:?}", value);
    }
}

fn fill_map_with_part_numbers_on_empty_positions(map: &EnginePartMap) -> EnginePartMap {
    let part_numbers = get_part_numbers(&map);

    let mut map = map.clone();

    part_numbers.iter().for_each(|(position, part)| {
        let (x, y) = position.clone();
        let len = part.length;

        for i in 0..len {
            map.insert((x + i, y), EnginePart::Number(part.clone()));
        }
    });

    map
}
