use itertools::Itertools;

use super::common::*;

const EMPTY_SPACE_LARGER_BY: u64 = 2;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let galaxy_positions = map
        .clone()
        .into_iter()
        .filter_map(|(position, pixel)| match pixel {
            Pixel::EmptySpace => None,
            Pixel::Galaxy => Some(position),
        })
        .collect_vec();

    let pairs = galaxy_positions
        .into_iter()
        .tuple_combinations::<(_, _)>()
        .collect_vec();

    let current_universe_max_x = map
        .keys()
        .into_iter()
        .map(|(x, _)| x)
        .max()
        .unwrap()
        .clone();
    let current_universe_max_y = map
        .keys()
        .into_iter()
        .map(|(_, y)| y)
        .max()
        .unwrap()
        .clone();

    let empty_rows = (0..=current_universe_max_y)
        .into_iter()
        .filter(|y| {
            let mut row_pixels = map.iter().filter(|((_, pixel_y), _)| pixel_y == y);
            let are_all_empty_space = row_pixels.all(|(_, value)| value == &Pixel::EmptySpace);

            are_all_empty_space
        })
        .collect_vec();

    let empty_columns = (0..=current_universe_max_x)
        .into_iter()
        .filter(|x| {
            let mut column_pixels = map.iter().filter(|((pixel_x, _), _)| pixel_x == x);
            let are_all_empty_space = column_pixels.all(|(_, value)| value == &Pixel::EmptySpace);

            are_all_empty_space
        })
        .collect_vec();

    let value = pairs
        .into_iter()
        .map(|(galaxy_position1, galaxy_position2)| {
            calculate_distance_between_galaxies(
                galaxy_position1,
                galaxy_position2,
                &empty_rows,
                &empty_columns,
                EMPTY_SPACE_LARGER_BY,
            )
        })
        .sum::<u64>();

    if use_sample {
        assert_eq!(value, 374);
    } else {
        println!("{:?}", value);
    }
}
