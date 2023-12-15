use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::multi::many1;
use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Ash,
    Rock,
}

pub type Position = (u8, u8);
pub type LavaMap = HashMap<Position, Cell>;

pub type LavaMaps = Vec<LavaMap>;

#[derive(Debug, Clone, PartialEq)]
pub enum LavaMapReflectionType {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LavaMapReflection {
    pub _type: LavaMapReflectionType,
    pub index: u8,
}

pub fn parse(input: &str) -> LavaMaps {
    let (_, maps) = many1(parse_map)(input).unwrap();
    maps
}

fn parse_map(input: &str) -> IResult<&str, LavaMap> {
    let mut lava_map = LavaMap::new();

    let mut y = 0;
    let mut x = 0;

    let mut remaining = input;

    loop {
        let (input, cell) =
            alt((map(line_ending, |_| None), map(parse_cell, |c| Some(c))))(remaining)?;

        remaining = input;

        match cell {
            Some(cell) => {
                lava_map.insert((x, y), cell);
                x += 1;
            }
            None => {
                y += 1;

                if x == 0 {
                    break;
                }

                x = 0;
            }
        }

        if remaining.is_empty() {
            break;
        }
    }

    Ok((remaining, lava_map))
}

fn parse_cell(input: &str) -> IResult<&str, Cell> {
    alt((map(tag("."), |_| Cell::Ash), map(tag("#"), |_| Cell::Rock)))(input)
}

pub fn sum_reflections(reflections: Vec<LavaMapReflection>) -> u32 {
    reflections
        .iter()
        .map(|reflection| match reflection._type {
            LavaMapReflectionType::Column => reflection.index as u32,
            LavaMapReflectionType::Row => reflection.index as u32 * 100,
        })
        .sum::<u32>()
}

pub fn find_reflections(lava_map: &LavaMap) -> Vec<LavaMapReflection> {
    let mut reflections = Vec::new();

    let min_x = 0;
    let max_x = lava_map.iter().map(|((x, _), _)| x).max().unwrap().clone();
    let min_y = 0;
    let max_y = lava_map.iter().map(|((_, y), _)| y).max().unwrap().clone();

    let mut columns = Vec::new();

    for x in min_x..=max_x {
        let column = (min_y..=max_y)
            .map(|y| lava_map.get(&(x, y)).unwrap().clone())
            .collect_vec();

        columns.push((x, column));
    }

    let columns_identical_positions = columns
        .iter()
        .map(|(x, column)| {
            let positions = columns
                .iter()
                .filter(|(other_x, _)| other_x != x)
                .filter(|(_, other_column)| other_column == column)
                .map(|(x, _)| x)
                .collect_vec();

            (x, positions)
        })
        .collect::<HashMap<_, _>>();

    let mut rows = Vec::new();

    for y in min_y..=max_y {
        let row = (min_x..=max_x)
            .map(|x| lava_map.get(&(x, y)).unwrap().clone())
            .collect_vec();

        rows.push((y, row));
    }

    let rows_identical_positions = rows
        .iter()
        .map(|(y, row)| {
            let positions = rows
                .iter()
                .filter(|(other_y, _)| other_y != y)
                .filter(|(_, other_row)| other_row == row)
                .map(|(x, _)| x)
                .collect_vec();

            (y, positions)
        })
        .collect::<HashMap<_, _>>();

    // from left to right
    let right_identical_positions = columns_identical_positions.get(&min_x).unwrap().clone();

    for right_index in right_identical_positions {
        let mut left_index = 0;
        let mut right_index = right_index.clone();

        loop {
            let diff = right_index - left_index;

            if diff == 1 {
                reflections.push(LavaMapReflection {
                    _type: LavaMapReflectionType::Column,
                    index: right_index,
                });
                break;
            } else {
                left_index += 1;
                right_index -= 1;

                let is_symmetric = columns_identical_positions
                    .get(&left_index)
                    .unwrap()
                    .clone()
                    .contains(&&right_index);

                if !is_symmetric {
                    break;
                }
            }
        }
    }

    // from right to left
    let left_identical_positions = columns_identical_positions.get(&max_x).unwrap().clone();

    for left_index in left_identical_positions {
        let mut left_index = left_index.clone();
        let mut right_index = max_x;

        loop {
            let diff = right_index - left_index;

            if diff == 1 {
                reflections.push(LavaMapReflection {
                    _type: LavaMapReflectionType::Column,
                    index: right_index,
                });
                break;
            } else {
                left_index += 1;
                right_index -= 1;

                let is_symmetric = columns_identical_positions
                    .get(&left_index)
                    .unwrap()
                    .clone()
                    .contains(&&right_index);

                if !is_symmetric {
                    break;
                }
            }
        }
    }

    // from top to bottom
    let bottom_identical_positions = rows_identical_positions.get(&min_y).unwrap().clone();

    for bottom_index in bottom_identical_positions {
        let mut top_index = min_y;
        let mut bottom_index = bottom_index.clone();

        loop {
            let diff = bottom_index - top_index;

            if diff == 1 {
                reflections.push(LavaMapReflection {
                    _type: LavaMapReflectionType::Row,
                    index: bottom_index,
                });
                break;
            } else {
                top_index += 1;
                bottom_index -= 1;

                let is_symmetric = rows_identical_positions
                    .get(&top_index)
                    .unwrap()
                    .clone()
                    .contains(&&bottom_index);

                if !is_symmetric {
                    break;
                }
            }
        }
    }

    // from bottom to top
    let top_identical_positions = rows_identical_positions.get(&max_y).unwrap().clone();

    for top_index in top_identical_positions {
        let mut top_index = top_index.clone();
        let mut bottom_index = max_y;

        loop {
            let diff = bottom_index - top_index;

            if diff == 1 {
                reflections.push(LavaMapReflection {
                    _type: LavaMapReflectionType::Row,
                    index: bottom_index,
                });
                break;
            } else {
                top_index += 1;
                bottom_index -= 1;

                let is_symmetric = rows_identical_positions
                    .get(&top_index)
                    .unwrap()
                    .clone()
                    .contains(&&bottom_index);

                if !is_symmetric {
                    break;
                }
            }
        }
    }

    reflections
}
