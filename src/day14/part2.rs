use itertools::Itertools;

use super::common::*;

const CYCLES: u32 = 1000000000;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let cycle_loop = get_cycle_loop(&map);
    let (cycled_maps, start) = cycle_loop.clone();

    let end = (cycled_maps.len() - 1) as u8;
    let cycle_length = end - start + 1;

    let remaining_cycles = CYCLES - start as u32 - 1;

    let cycle_map_index = start as u32 + (remaining_cycles % cycle_length as u32);

    let map = &cycled_maps[cycle_map_index as usize];

    print_map(map.clone());

    let value = calculate_total_load(map);

    if use_sample {
        assert_eq!(value, 64);
    } else {
        println!("{}", value);
    }
}

fn get_cycle_loop(map: &LavaMap) -> (Vec<LavaMap>, u8) {
    let mut cycled_maps = Vec::new();
    let mut next_map = map.clone();

    loop {
        next_map = get_next_lava_map(&next_map);

        for (start, map_start) in cycled_maps.iter().enumerate() {
            if map_start == &next_map {
                return (cycled_maps, start as u8);
            }
        }

        cycled_maps.push(next_map.clone());
    }
}

fn get_next_lava_map(map: &LavaMap) -> LavaMap {
    let mut next_map = map.clone();

    let max_x = map.iter().map(|((x, _), _)| x).max().unwrap().clone();
    let max_y = map.iter().map(|((_, y), _)| y).max().unwrap().clone();

    // apply NORTH move
    for x in 0..=max_x {
        loop {
            let mut has_change = false;

            for y in (1..=max_y).rev() {
                let current_position = (x, y);
                let next_position = (x, y - 1);

                let current_tile = next_map.get(&current_position).unwrap();
                let next_tile = next_map.get(&next_position).unwrap();

                if current_tile == &Tile::RoundedRock && next_tile == &Tile::Empty {
                    next_map.insert(current_position, Tile::Empty);
                    next_map.insert(next_position, Tile::RoundedRock);

                    has_change = true;
                }
            }

            if !has_change {
                break;
            }
        }
    }

    // apply WEST move
    for y in 0..=max_y {
        loop {
            let mut has_change = false;

            for x in (1..=max_x).rev() {
                let current_position = (x, y);
                let next_position = (x - 1, y);

                let current_tile = next_map.get(&current_position).unwrap();
                let next_tile = next_map.get(&next_position).unwrap();

                if current_tile == &Tile::RoundedRock && next_tile == &Tile::Empty {
                    next_map.insert(current_position, Tile::Empty);
                    next_map.insert(next_position, Tile::RoundedRock);

                    has_change = true;
                }
            }

            if !has_change {
                break;
            }
        }
    }

    // apply SOUTH move
    for x in 0..=max_x {
        loop {
            let mut has_change = false;

            for y in 0..max_y {
                let current_position = (x, y);
                let next_position = (x, y + 1);

                let current_tile = next_map.get(&current_position).unwrap();
                let next_tile = next_map.get(&next_position).unwrap();

                if current_tile == &Tile::RoundedRock && next_tile == &Tile::Empty {
                    next_map.insert(current_position, Tile::Empty);
                    next_map.insert(next_position, Tile::RoundedRock);

                    has_change = true;
                }
            }

            if !has_change {
                break;
            }
        }
    }

    // apply EAST move
    for y in 0..=max_y {
        loop {
            let mut has_change = false;

            for x in 0..max_x {
                let current_position = (x, y);
                let next_position = (x + 1, y);

                let current_tile = next_map.get(&current_position).unwrap();
                let next_tile = next_map.get(&next_position).unwrap();

                if current_tile == &Tile::RoundedRock && next_tile == &Tile::Empty {
                    next_map.insert(current_position, Tile::Empty);
                    next_map.insert(next_position, Tile::RoundedRock);

                    has_change = true;
                }
            }

            if !has_change {
                break;
            }
        }
    }

    next_map
}

fn calculate_total_load(map: &LavaMap) -> u32 {
    let max_y = map.iter().map(|((_, y), _)| y).max().unwrap().clone();

    let mut max_load = max_y + 1;
    let mut value = 0;

    for y in 0..=max_y {
        let rounded_rocks = map
            .iter()
            .filter(|((_, tile_y), _)| tile_y == &y)
            .filter(|(_, tile)| tile == &&Tile::RoundedRock)
            .collect_vec();

        value += rounded_rocks.len() as u32 * max_load as u32;

        max_load -= 1;
    }

    value
}

fn print_map(map: LavaMap) {
    let max_x = map.iter().map(|((x, _), _)| x).max().unwrap().clone();
    let max_y = map.iter().map(|((_, y), _)| y).max().unwrap().clone();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let cell = map.get(&(x, y)).unwrap();

            let cell = match cell {
                Tile::Empty => '.',
                Tile::CubeRock => '#',
                Tile::RoundedRock => 'O',
            };

            print!("{}", cell);
        }

        println!("");
    }
}
