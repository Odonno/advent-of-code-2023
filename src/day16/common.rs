use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub type Position = (i8, i8);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Beam {
    pub position: Position,
    pub direction: Direction,
}

type IsMirrorClockwise = bool;

#[derive(Debug, Clone, PartialEq)]
pub enum SplitterOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    Empty,
    Mirror(IsMirrorClockwise),
    Splitter(SplitterOrientation),
}

pub type Grid = HashMap<Position, Tile>;

pub fn parse(input: &str) -> Grid {
    let mut map = Grid::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let tile = match char {
                '.' => Tile::Empty,
                '/' => Tile::Mirror(false),
                '\\' => Tile::Mirror(true),
                '-' => Tile::Splitter(SplitterOrientation::Horizontal),
                '|' => Tile::Splitter(SplitterOrientation::Vertical),
                _ => panic!("Unknown character: {}", char),
            };

            map.insert((x as i8, y as i8), tile);
        }
    }

    map
}

pub fn extract_visited_beams(map: &Grid, entry_beam: &Beam) -> HashSet<Beam> {
    let mut beams = vec![entry_beam.clone()];
    let mut visited_beams = HashSet::new();

    loop {
        let mut new_beams = Vec::new();

        for beam in beams {
            if entry_beam != &beam {
                visited_beams.insert(beam.clone());
            }

            let next_tile_position = get_next_direction(&beam.position, &beam.direction);
            let next_tile = map.get(&next_tile_position);

            if let Some(next_tile) = next_tile {
                match next_tile {
                    Tile::Empty => {
                        new_beams.push(Beam {
                            position: next_tile_position,
                            direction: beam.direction,
                        });
                    }
                    Tile::Mirror(clockwise) => {
                        let mirror_position = next_tile_position;

                        let new_direction = if clockwise == &true {
                            match beam.direction {
                                Direction::Top => Direction::Left,
                                Direction::Right => Direction::Bottom,
                                Direction::Bottom => Direction::Right,
                                Direction::Left => Direction::Top,
                            }
                        } else {
                            match beam.direction {
                                Direction::Top => Direction::Right,
                                Direction::Left => Direction::Bottom,
                                Direction::Bottom => Direction::Left,
                                Direction::Right => Direction::Top,
                            }
                        };

                        new_beams.push(Beam {
                            position: mirror_position,
                            direction: new_direction,
                        });
                    }
                    Tile::Splitter(orientation) => match orientation {
                        SplitterOrientation::Horizontal => {
                            if beam.direction == Direction::Right
                                || beam.direction == Direction::Left
                            {
                                new_beams.push(Beam {
                                    position: next_tile_position,
                                    direction: beam.direction,
                                });
                            } else {
                                let splitter_position = next_tile_position;

                                new_beams.push(Beam {
                                    position: splitter_position,
                                    direction: Direction::Left,
                                });
                                new_beams.push(Beam {
                                    position: splitter_position,
                                    direction: Direction::Right,
                                });
                            }
                        }
                        SplitterOrientation::Vertical => {
                            if beam.direction == Direction::Top
                                || beam.direction == Direction::Bottom
                            {
                                new_beams.push(Beam {
                                    position: next_tile_position,
                                    direction: beam.direction,
                                });
                            } else {
                                let splitter_position = next_tile_position;

                                new_beams.push(Beam {
                                    position: splitter_position,
                                    direction: Direction::Top,
                                });
                                new_beams.push(Beam {
                                    position: splitter_position,
                                    direction: Direction::Bottom,
                                });
                            }
                        }
                    },
                }
            }
        }

        if new_beams.len() == 0 {
            break;
        }

        beams = new_beams
            .into_iter()
            .filter(|beam| !visited_beams.contains(beam))
            .collect();
    }
    visited_beams
}

pub fn get_next_direction(position: &Position, direction: &Direction) -> (i8, i8) {
    let (x, y) = position.clone();

    match direction {
        Direction::Top => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
        Direction::Bottom => (x, y + 1),
    }
}

pub fn get_energized_positions(visited_beams: HashSet<Beam>) -> HashSet<(i8, i8)> {
    visited_beams
        .iter()
        .unique_by(|beam| beam.position)
        .map(|beam| beam.position)
        .collect::<HashSet<_>>()
}

pub fn print_energized_map(map: &Grid, energized_positions: &HashSet<Position>) {
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();
    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let position = (x, y);

            if energized_positions.contains(&position) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
