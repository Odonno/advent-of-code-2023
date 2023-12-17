use itertools::Itertools;

use super::common::*;

const MIN_STRAIGHT: usize = 4;
const MAX_STRAIGHT: usize = 10;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();

    let mut min_value = u32::MAX;

    for constraint in (MIN_STRAIGHT - 1)..=(MAX_STRAIGHT - 1) {
        println!("Constraint: {}", constraint);

        let constraint = constraint as u8;

        if constraint > max_x || constraint > max_y {
            continue;
        }

        let left_target = (max_x - constraint, max_y);
        let top_target = (max_x, max_y - constraint);

        let paths_from_right_to_left = find_closest_path(
            ((1, 0), Direction::Right),
            left_target,
            &map,
            MIN_STRAIGHT,
            MAX_STRAIGHT,
            Some(Direction::Right),
        );
        let paths_from_right_to_top = find_closest_path(
            ((1, 0), Direction::Right),
            top_target,
            &map,
            MIN_STRAIGHT,
            MAX_STRAIGHT,
            Some(Direction::Down),
        );
        let paths_from_down_to_left = find_closest_path(
            ((0, 1), Direction::Down),
            left_target,
            &map,
            MIN_STRAIGHT,
            MAX_STRAIGHT,
            Some(Direction::Right),
        );
        let paths_from_down_to_top = find_closest_path(
            ((0, 1), Direction::Down),
            top_target,
            &map,
            MIN_STRAIGHT,
            MAX_STRAIGHT,
            Some(Direction::Down),
        );

        let to_left_paths = paths_from_right_to_left
            .into_iter()
            .chain(paths_from_down_to_left.into_iter())
            .collect_vec();

        let to_top_paths = paths_from_right_to_top
            .into_iter()
            .chain(paths_from_down_to_top.into_iter())
            .collect_vec();

        let mut remaining_to_left_target_value = 0;
        for diff in 0..constraint {
            let position = (max_x - diff, max_y);
            remaining_to_left_target_value += map.get(&position).unwrap().clone();
        }

        let mut remaining_to_top_target_value = 0;
        for diff in 0..constraint {
            let position = (max_x, max_y - diff);
            remaining_to_top_target_value += map.get(&position).unwrap().clone();
        }

        let to_left_values = to_left_paths
            .iter()
            .map(|path| {
                let value = path
                    .iter()
                    .map(|position| map.get(position).unwrap().clone() as u32)
                    .sum::<u32>();

                value + remaining_to_left_target_value as u32
            })
            .collect_vec();

        for value in to_left_values {
            if value < min_value {
                min_value = value;
            }
        }

        let to_top_values = to_top_paths
            .iter()
            .map(|path| {
                let value = path
                    .iter()
                    .map(|position| map.get(position).unwrap().clone() as u32)
                    .sum::<u32>();

                value + remaining_to_top_target_value as u32
            })
            .collect_vec();

        for value in to_top_values {
            if value < min_value {
                min_value = value;
            }
        }
    }

    let value = min_value;

    if use_sample {
        assert_eq!(value, 71);
    } else {
        println!("{}", value);
    }
}
