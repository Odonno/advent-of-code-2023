use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;

pub type Position = (u8, u8);

#[derive(Debug, Clone, PartialEq)]
pub enum Pixel {
    EmptySpace,
    Galaxy,
}

pub type UniverseMap = HashMap<Position, Pixel>;

pub fn parse(input: &str) -> UniverseMap {
    let mut map = UniverseMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let pixel = match char {
                '.' => Pixel::EmptySpace,
                '#' => Pixel::Galaxy,
                _ => panic!("Impossible char"),
            };

            map.insert((x as u8, y as u8), pixel);
        });
    });

    map
}

pub fn calculate_distance_between_galaxies(
    p1: Position,
    p2: Position,
    empty_rows: &Vec<u8>,
    empty_columns: &Vec<u8>,
    empty_space_larger_by: u64,
) -> u64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    let x = (x1 as u64).abs_diff(x2 as u64);
    let y = (y1 as u64).abs_diff(y2 as u64);

    let min_x = min(x1, x2);
    let max_x = max(x1, x2);

    let min_y = min(y1, y2);
    let max_y = max(y1, y2);

    let between_empty_rows_count = empty_rows
        .into_iter()
        .filter(|row| &&min_y < row && row < &&max_y)
        .collect_vec()
        .len() as u64;
    let between_empty_columns_count = empty_columns
        .into_iter()
        .filter(|column| &&min_x < column && column < &&max_x)
        .collect_vec()
        .len() as u64;

    let total_empty_row_or_column = between_empty_rows_count + between_empty_columns_count;

    x + y + ((empty_space_larger_by - 1) * total_empty_row_or_column)
}
