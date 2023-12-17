use itertools::Itertools;
use std::collections::HashMap;

pub type Position = (u8, u8);
pub type HeatLoss = u8;
pub type LavaMap = HashMap<Position, HeatLoss>;

pub fn parse(input: &str) -> LavaMap {
    let mut map = LavaMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let heat_loss = char.to_digit(10).unwrap() as u8;
            map.insert((x as u8, y as u8), heat_loss);
        }
    }

    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub type Move = (Position, Direction);

pub type Path = Vec<Position>;

pub type ParentMap = HashMap<Move, (Path, Direction)>;

pub type Score = u32;

pub type GScoreMap = HashMap<Move, Score>;
pub type FScoreMap = HashMap<Position, Score>;

pub fn find_closest_path(
    origin: Move,
    target: Position,
    map: &LavaMap,
    min_straigth: usize,
    max_straight: usize,
    final_direction: Option<Direction>,
) -> Vec<Path> {
    let mut open_set = vec![origin];

    let mut g_scores = GScoreMap::new();
    let mut f_scores = FScoreMap::new();

    let mut parents = ParentMap::new();

    let (origin_position, _) = origin;

    let total_distance = calculate_distance(origin_position, target);

    g_scores.insert(origin, 0);
    f_scores.insert(origin_position, total_distance);

    let mut paths = Vec::new();

    while !open_set.is_empty() {
        let current = open_set
            .iter()
            .min_by(|a, b| f_scores.get(&a.0).unwrap().cmp(f_scores.get(&b.0).unwrap()))
            .unwrap()
            .clone();

        let (current_position, current_direction) = current;

        open_set.retain(|_move| _move != &current);

        if current_position == target {
            if let Some(final_direction) = final_direction {
                if current_direction == final_direction {
                    paths.push(reconstruct_path(&parents, current));
                }
            } else {
                paths.push(reconstruct_path(&parents, current));
            }

            continue;
        }

        let neighbors = find_neighbors_path(current, map, min_straigth, max_straight);

        for (straight_line, neighbor) in neighbors {
            let (neighbor_position, neighbor_direction) = neighbor;

            let value = straight_line
                .iter()
                .map(|position| map.get(position).unwrap().clone() as u32)
                .sum::<u32>();

            let tentative_g_score = g_scores.get(&current).unwrap().clone() + value;
            let neighbor_g_score = g_scores.get(&neighbor).unwrap_or(&Score::MAX).clone();

            if tentative_g_score < neighbor_g_score {
                parents.insert(neighbor, (straight_line, current_direction));
                g_scores.insert(neighbor, tentative_g_score);

                let h_score = match neighbor_direction {
                    Direction::Down | Direction::Right => {
                        calculate_distance(neighbor_position, target)
                    }
                    Direction::Up | Direction::Left => {
                        calculate_distance(neighbor_position, target) + 1
                    }
                };

                let f_score = tentative_g_score + h_score;
                f_scores.insert(neighbor_position, f_score);

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    paths
}

fn calculate_distance(origin: Position, target: Position) -> u32 {
    let (x1, y1) = origin;
    let (x2, y2) = target;

    let x_distance = (x1 as i32 - x2 as i32).abs() as u32;
    let y_distance = (y1 as i32 - y2 as i32).abs() as u32;

    x_distance + y_distance
}

fn reconstruct_path(parents: &ParentMap, mut current: Move) -> Path {
    let mut path = vec![current.0];

    while parents.contains_key(&current) {
        let line = parents.get(&current).unwrap().clone();

        current = (line.0.first().unwrap().clone(), line.1);

        for position in line.0 {
            path.push(position);
        }
    }

    path
}

fn find_neighbors_path(
    _move: Move,
    map: &LavaMap,
    min_straigth: usize,
    max_straight: usize,
) -> Vec<(Vec<Position>, Move)> {
    let (position, direction) = _move;
    let (x, y) = position;

    let straight_positions = match direction {
        Direction::Up => {
            let mut p = vec![];
            let mut acc = vec![];

            for diff in 0..max_straight {
                let new_y = y as i16 - diff as i16;
                if new_y < 0 {
                    break;
                }

                let position = (x, new_y as u8);
                acc.push(position);

                if min_straigth <= acc.len() && acc.len() <= max_straight {
                    p.push(acc.clone());
                }
            }

            p
        }
        Direction::Down => {
            let mut p = vec![];
            let mut acc = vec![];

            for diff in 0..max_straight {
                let position = (x, y + diff as u8);
                acc.push(position);

                if min_straigth <= acc.len() && acc.len() <= max_straight {
                    p.push(acc.clone());
                }
            }

            p
        }
        Direction::Left => {
            let mut p = vec![];
            let mut acc = vec![];

            for diff in 0..max_straight {
                let new_x = x as i16 - diff as i16;
                if new_x < 0 {
                    break;
                }

                let position = (new_x as u8, y);
                acc.push(position);

                if min_straigth <= acc.len() && acc.len() <= max_straight {
                    p.push(acc.clone());
                }
            }

            p
        }
        Direction::Right => {
            let mut p = vec![];
            let mut acc = vec![];

            for diff in 0..max_straight {
                let position = (x + diff as u8, y);
                acc.push(position);

                if min_straigth <= acc.len() && acc.len() <= max_straight {
                    p.push(acc.clone());
                }
            }

            p
        }
    };

    let next_moves = straight_positions
        .into_iter()
        .flat_map(|positions| {
            let last_position = positions.last().unwrap().clone();
            let (x, y) = last_position;

            let adjacent_moves = match direction {
                Direction::Up | Direction::Down if x == 0 => vec![((x + 1, y), Direction::Right)],
                Direction::Up | Direction::Down => vec![
                    ((x - 1, y), Direction::Left),
                    ((x + 1, y), Direction::Right),
                ],
                Direction::Left | Direction::Right if y == 0 => {
                    vec![((x, y + 1), Direction::Down)]
                }
                Direction::Left | Direction::Right => {
                    vec![((x, y - 1), Direction::Up), ((x, y + 1), Direction::Down)]
                }
            };

            adjacent_moves
                .into_iter()
                .map(|m| (positions.clone(), m))
                .collect_vec()
        })
        .filter(|(_, (position, _))| map.get(&position).is_some())
        .collect_vec();

    next_moves
}

pub fn print_path(map: &LavaMap, path: &Path) {
    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let position = path
                .iter()
                .find(|(path_x, path_y)| path_x == &x && path_y == &y);

            let char = match position {
                Some(_) => map
                    .get(&(x, y))
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap(),
                None => '.',
            };

            print!("{}", char);
        }

        println!();
    }
}
