use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::char;
use nom::{combinator::map, IResult};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BendDirection {
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Pipe(PipeOrientation),
    Bend(BendDirection),
    Ground,
    StartingPosition,
}

pub type Position = (u8, u8);

pub type Map = HashMap<Position, Tile>;

pub type Path = Vec<(Direction, Position)>;

#[derive(Debug, Clone)]
pub struct Input {
    pub map: Map,
    pub starting_position: Position,
}

pub fn parse(input: &str) -> Input {
    let map = parse_map(input).unwrap().1;

    let starting_position = map
        .clone()
        .into_iter()
        .find(|(_, tile)| tile == &Tile::StartingPosition)
        .unwrap()
        .0;

    let bend_at_starting_position = detect_bend_at_starting_position(&map, starting_position);

    let mut map = map;
    map.entry(starting_position)
        .and_modify(|tile| *tile = bend_at_starting_position);

    Input {
        map,
        starting_position,
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let mut map = Map::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let y = y as u8;
        let mut x = 0;

        let mut input = line;

        while let Ok((remaining, tile)) = parse_char(input) {
            map.insert((x, y), tile);

            input = remaining;
            x += 1;
        }
    });

    Ok((input, map))
}

fn parse_char(input: &str) -> IResult<&str, Tile> {
    let (input, part) = alt((
        map(char('|'), |_| Tile::Pipe(PipeOrientation::Vertical)),
        map(char('-'), |_| Tile::Pipe(PipeOrientation::Horizontal)),
        map(char('L'), |_| Tile::Bend(BendDirection::NorthEast)),
        map(char('J'), |_| Tile::Bend(BendDirection::NorthWest)),
        map(char('7'), |_| Tile::Bend(BendDirection::SouthWest)),
        map(char('F'), |_| Tile::Bend(BendDirection::SouthEast)),
        map(char('.'), |_| Tile::Ground),
        map(char('S'), |_| Tile::StartingPosition),
    ))(input)?;

    Ok((input, part))
}

fn detect_bend_at_starting_position(map: &Map, starting_position: Position) -> Tile {
    let connector_positions = find_loop_connector_positions(map, starting_position);

    let (d1, _) = connector_positions[0];
    let (d2, _) = connector_positions[1];

    if d1 == Direction::West {
        if d2 == Direction::North {
            Tile::Bend(BendDirection::NorthWest)
        } else {
            Tile::Bend(BendDirection::SouthWest)
        }
    } else {
        if d2 == Direction::North {
            Tile::Bend(BendDirection::NorthEast)
        } else {
            Tile::Bend(BendDirection::SouthEast)
        }
    }
}

fn find_loop_connector_positions(map: &Map, position: Position) -> Vec<(Direction, Position)> {
    let (x, y) = position;

    let mut positions = Vec::new();

    let west_tile = if x <= 0 { None } else { map.get(&(x - 1, y)) };
    let east_tile = map.get(&(x + 1, y));
    let north_tile = if y <= 0 { None } else { map.get(&(x, y - 1)) };
    let south_tile = map.get(&(x, y + 1));

    if let Some(tile) = west_tile {
        if tile == &Tile::Pipe(PipeOrientation::Horizontal)
            || tile == &Tile::Bend(BendDirection::NorthEast)
            || tile == &Tile::Bend(BendDirection::SouthEast)
        {
            positions.push((Direction::West, (x - 1, y)));
        }
    }
    if let Some(tile) = east_tile {
        if tile == &Tile::Pipe(PipeOrientation::Horizontal)
            || tile == &Tile::Bend(BendDirection::NorthWest)
            || tile == &Tile::Bend(BendDirection::SouthWest)
        {
            positions.push((Direction::East, (x + 1, y)));
        }
    }
    if let Some(tile) = north_tile {
        if tile == &Tile::Pipe(PipeOrientation::Vertical)
            || tile == &Tile::Bend(BendDirection::SouthEast)
            || tile == &Tile::Bend(BendDirection::SouthWest)
        {
            positions.push((Direction::North, (x, y - 1)));
        }
    }
    if let Some(tile) = south_tile {
        if tile == &Tile::Pipe(PipeOrientation::Vertical)
            || tile == &Tile::Bend(BendDirection::NorthEast)
            || tile == &Tile::Bend(BendDirection::NorthWest)
        {
            positions.push((Direction::South, (x, y + 1)));
        }
    }

    positions
}

fn find_next_path(map: &Map, path: &Path) -> Vec<Path> {
    let mut new_paths = Vec::new();

    let last_direction = path.last().unwrap().0.clone();
    let last_position = path.last().unwrap().1.clone();

    let next_position = match last_direction {
        Direction::North => (last_position.0, last_position.1 - 1),
        Direction::South => (last_position.0, last_position.1 + 1),
        Direction::East => (last_position.0 + 1, last_position.1),
        Direction::West => (last_position.0 - 1, last_position.1),
    };

    let tile = map.get(&next_position).unwrap();

    match tile {
        Tile::Pipe(pipe_orientation) => {
            let is_correct_orientation = match last_direction {
                Direction::North | Direction::South => {
                    pipe_orientation == &PipeOrientation::Vertical
                }
                Direction::East | Direction::West => {
                    pipe_orientation == &PipeOrientation::Horizontal
                }
            };

            if is_correct_orientation {
                new_paths.push(
                    path.clone()
                        .into_iter()
                        .chain(vec![(last_direction, next_position)])
                        .collect_vec(),
                );
            }
        }
        Tile::Bend(bend_direction) => {
            let next_direction = match (last_direction, bend_direction) {
                (Direction::North, BendDirection::SouthEast) => Some(Direction::East),
                (Direction::North, BendDirection::SouthWest) => Some(Direction::West),
                (Direction::South, BendDirection::NorthEast) => Some(Direction::East),
                (Direction::South, BendDirection::NorthWest) => Some(Direction::West),
                (Direction::East, BendDirection::NorthWest) => Some(Direction::North),
                (Direction::East, BendDirection::SouthWest) => Some(Direction::South),
                (Direction::West, BendDirection::NorthEast) => Some(Direction::North),
                (Direction::West, BendDirection::SouthEast) => Some(Direction::South),
                _ => None,
            };

            if let Some(next_direction) = next_direction {
                new_paths.push(
                    path.clone()
                        .into_iter()
                        .chain(vec![(next_direction, next_position)])
                        .collect_vec(),
                );
            }
        }
        Tile::Ground => {}
        _ => panic!("Invalid tile"),
    }

    new_paths
}

pub fn find_correct_path(input: &Input) -> Path {
    let starting_tile = input.map.get(&input.starting_position).unwrap();

    let mut paths: Vec<Path> = Vec::new();

    match starting_tile {
        Tile::Bend(bend_direction) => match bend_direction {
            BendDirection::NorthEast | BendDirection::NorthWest => {
                paths.push(vec![(Direction::North, input.starting_position)])
            }
            BendDirection::SouthEast | BendDirection::SouthWest => {
                paths.push(vec![(Direction::South, input.starting_position)])
            }
        },
        _ => panic!("Starting tile is not a bend"),
    }

    loop {
        paths = paths
            .into_iter()
            .flat_map(|path| find_next_path(&input.map, &path))
            .collect_vec();

        let correct_path = paths.clone().into_iter().find(|path| {
            path.into_iter()
                .filter(|p| p.1 == input.starting_position)
                .collect_vec()
                .len()
                == 2
        });

        if let Some(correct_path) = correct_path {
            paths = vec![correct_path];
            break;
        }
    }

    paths[0].clone()
}
