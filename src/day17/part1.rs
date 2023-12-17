use itertools::Itertools;

use super::common::*;

const MIN_STRAIGHT: usize = 1;
const MAX_STRAIGHT: usize = 3;
const FINAL_DIRECTION: Option<Direction> = None;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();

    let target = (max_x, max_y);

    let paths_from_right = find_closest_path(
        ((1, 0), Direction::Right),
        target,
        &map,
        MIN_STRAIGHT,
        MAX_STRAIGHT,
        FINAL_DIRECTION,
    );
    let paths_from_down = find_closest_path(
        ((0, 1), Direction::Down),
        target,
        &map,
        MIN_STRAIGHT,
        MAX_STRAIGHT,
        FINAL_DIRECTION,
    );

    let all_paths = paths_from_right
        .into_iter()
        .chain(paths_from_down.into_iter())
        .collect_vec();

    for path in all_paths.iter() {
        print_path(&map, &path);
        println!("-----------------");
    }

    let value = all_paths
        .iter()
        .map(|path| {
            let value = path
                .iter()
                .map(|position| map.get(position).unwrap().clone() as u32)
                .sum::<u32>();

            value
        })
        .min()
        .unwrap();

    if use_sample {
        assert_eq!(value, 102);
    } else {
        println!("{}", value);
    }
}
