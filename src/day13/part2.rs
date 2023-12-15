use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let maps = parse(input);

    let reflections = maps
        .into_iter()
        .map(|map| {
            let default_reflections = find_reflections(&map);

            let max_x = map.iter().map(|((x, _), _)| x).max().unwrap().clone();
            let max_y = map.iter().map(|((_, y), _)| y).max().unwrap().clone();

            for x in 0..=max_x {
                for y in 0..=max_y {
                    let mut smudge_map = map.clone();

                    let smudge_cell = smudge_map.get(&(x, y)).unwrap();

                    match smudge_cell {
                        Cell::Ash => {
                            smudge_map.insert((x, y), Cell::Rock);
                        }
                        Cell::Rock => {
                            smudge_map.insert((x, y), Cell::Ash);
                        }
                    }

                    let other_reflections = find_reflections(&smudge_map)
                        .into_iter()
                        .filter(|r| !default_reflections.contains(r))
                        .collect_vec();

                    if other_reflections.len() > 0 {
                        return other_reflections.first().unwrap().clone();
                    }
                }
            }

            panic!("no smudge found");
        })
        .collect_vec();

    let value = sum_reflections(reflections);

    if use_sample {
        assert_eq!(value, 400);
    } else {
        println!("{:?}", value);
    }
}
