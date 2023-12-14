use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let value = calculate_total_load(map);

    if use_sample {
        assert_eq!(value, 136);
    } else {
        println!("{}", value);
    }
}

fn calculate_total_load(map: LavaMap) -> u32 {
    let max_x = map.iter().map(|((x, _), _)| x).max().unwrap().clone();
    let max_y = map.iter().map(|((_, y), _)| y).max().unwrap().clone();

    let max_load = max_y + 1;

    let mut value = 0;

    for x in 0..=max_x {
        let column = map
            .iter()
            .filter(|((tile_x, _), _)| tile_x == &x)
            .sorted_by(|((_, y1), _), ((_, y2), _)| y1.cmp(y2))
            .collect_vec();

        let cube_rock_positions = column
            .iter()
            .positions(|(_, tile)| tile == &&Tile::CubeRock)
            .collect_vec();

        let rounded_rocks = column
            .iter()
            .filter(|(_, tile)| tile == &&Tile::RoundedRock)
            .collect_vec();

        if cube_rock_positions.is_empty() {
            let mut current_load = max_load;

            for _ in 0..rounded_rocks.len() {
                value += current_load as u32;
                current_load -= 1;
            }

            continue;
        }

        let mut last_cube_rock_position = None;

        for cube_rock_position in cube_rock_positions {
            if let Some(last_position) = last_cube_rock_position {
                let rounded_rocks_between = rounded_rocks
                    .iter()
                    .filter(|((_, y), _)| {
                        &(last_position as u8) < y && y < &(cube_rock_position as u8)
                    })
                    .count();

                let mut current_load = max_load - (last_position as u8) - 1;

                for _ in 0..rounded_rocks_between {
                    value += current_load as u32;
                    current_load -= 1;
                }
            } else {
                let rounded_rocks_before = rounded_rocks
                    .iter()
                    .filter(|((_, y), _)| y < &(cube_rock_position as u8))
                    .count();

                let mut current_load = max_load;

                for _ in 0..rounded_rocks_before {
                    value += current_load as u32;
                    current_load -= 1;
                }
            }

            last_cube_rock_position = Some(cube_rock_position);
        }

        if let Some(last_position) = last_cube_rock_position {
            let rounded_rocks_after = rounded_rocks
                .iter()
                .filter(|((_, y), _)| &(last_position as u8) < y)
                .count();

            let mut current_load = max_load - (last_position as u8) - 1;

            for _ in 0..rounded_rocks_after {
                value += current_load as u32;
                current_load -= 1;
            }
        }
    }
    value
}
