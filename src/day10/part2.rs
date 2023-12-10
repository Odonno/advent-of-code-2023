use itertools::Itertools;
use std::cmp::{max, min};

use super::common::*;

#[derive(Debug, Clone)]
struct PositionWatcher {
    from: Position,
    to: Position,
    direction: Direction,
}

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);

    let correct_path = find_correct_path(&input);

    let corners = correct_path
        .clone()
        .into_iter()
        .dedup_by(|a, b| a.0 == b.0)
        .collect_vec();

    let segments = corners
        .into_iter()
        .skip(1)
        .circular_tuple_windows()
        .map(|(p1, p2)| (p1.0, p1.1, p2.1))
        .collect_vec();

    let mut clockwises = correct_path
        .clone()
        .into_iter()
        .take(correct_path.len() - 1)
        .map(|p| p.0)
        .dedup()
        .collect_vec()
        .into_iter()
        .circular_tuple_windows()
        .map(|(d1, d2)| match (d1, d2) {
            (Direction::North, Direction::East) => true,
            (Direction::North, Direction::West) => false,
            (Direction::South, Direction::East) => false,
            (Direction::South, Direction::West) => true,
            (Direction::East, Direction::North) => false,
            (Direction::East, Direction::South) => true,
            (Direction::West, Direction::North) => true,
            (Direction::West, Direction::South) => false,
            _ => panic!("Invalid direction combination"),
        });

    let mut position_watchers = Vec::new();
    let current_path_direction = segments.first().unwrap().0.clone();
    let mut current_watch_direction = current_path_direction.clone();

    for segment in segments {
        let (_, from, to) = segment;

        let is_clockwise = clockwises.next().unwrap();

        current_watch_direction = match (current_watch_direction, is_clockwise) {
            (Direction::North, true) => Direction::East,
            (Direction::North, false) => Direction::West,
            (Direction::South, true) => Direction::West,
            (Direction::South, false) => Direction::East,
            (Direction::East, true) => Direction::South,
            (Direction::East, false) => Direction::North,
            (Direction::West, true) => Direction::North,
            (Direction::West, false) => Direction::South,
        };

        let watcher = PositionWatcher {
            from,
            to,
            direction: current_watch_direction.clone(),
        };

        position_watchers.push(watcher);
    }

    let points_in_correct_path = correct_path.into_iter().map(|p| p.1).collect_vec();

    let max_x = input.map.keys().map(|p| p.0).max().unwrap();
    let max_y = input.map.keys().map(|p| p.1).max().unwrap();

    let mut enclosed_points = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let position = (x, y);

            if points_in_correct_path.contains(&position) {
                continue;
            }

            if has_left_segment(position, &position_watchers)
                && has_right_segment(position, max_x, &position_watchers)
                && has_top_segment(position, &position_watchers)
                && has_bottom_segment(position, max_y, &position_watchers)
            {
                enclosed_points += 1;
            }
        }
    }

    let value = enclosed_points;

    if use_sample {
        assert_eq!(value, 8);
    } else {
        println!("{:?}", value);
    }
}

fn has_left_segment(position: Position, position_watchers: &Vec<PositionWatcher>) -> bool {
    let (original_x, original_y) = position;

    for x in (0..original_x).rev() {
        let watcher = position_watchers
            .iter()
            .filter(|w| w.direction == Direction::East || w.direction == Direction::West)
            .find(|w| {
                let (from_x, from_y) = w.from;
                let (to_x, to_y) = w.to;

                let min_y = min(from_y, to_y);
                let max_y = max(from_y, to_y);

                from_x == x && to_x == x && min_y <= original_y && original_y <= max_y
            });

        if let Some(watcher) = watcher {
            return watcher.direction == Direction::East;
        }
    }

    false
}

fn has_right_segment(
    position: Position,
    max_x: u8,
    position_watchers: &Vec<PositionWatcher>,
) -> bool {
    let (original_x, original_y) = position;

    for x in (original_x + 1)..max_x {
        let watcher = position_watchers
            .iter()
            .filter(|w| w.direction == Direction::East || w.direction == Direction::West)
            .find(|w| {
                let (from_x, from_y) = w.from;
                let (to_x, to_y) = w.to;

                let min_y = min(from_y, to_y);
                let max_y = max(from_y, to_y);

                from_x == x && to_x == x && min_y <= original_y && original_y <= max_y
            });

        if let Some(watcher) = watcher {
            return watcher.direction == Direction::West;
        }
    }

    false
}

fn has_top_segment(position: Position, position_watchers: &Vec<PositionWatcher>) -> bool {
    let (original_x, original_y) = position;

    for y in (0..original_y).rev() {
        let watcher = position_watchers
            .iter()
            .filter(|w| w.direction == Direction::North || w.direction == Direction::South)
            .find(|w| {
                let (from_x, from_y) = w.from;
                let (to_x, to_y) = w.to;

                let min_x = min(from_x, to_x);
                let max_x = max(from_x, to_x);

                from_y == y && to_y == y && min_x <= original_x && original_x <= max_x
            });

        if let Some(watcher) = watcher {
            return watcher.direction == Direction::South;
        }
    }

    false
}

fn has_bottom_segment(
    position: Position,
    max_y: u8,
    position_watchers: &Vec<PositionWatcher>,
) -> bool {
    let (original_x, original_y) = position;

    for y in (original_y + 1)..max_y {
        let watcher = position_watchers
            .iter()
            .filter(|w| w.direction == Direction::North || w.direction == Direction::South)
            .find(|w| {
                let (from_x, from_y) = w.from;
                let (to_x, to_y) = w.to;

                let min_x = min(from_x, to_x);
                let max_x = max(from_x, to_x);

                from_y == y && to_y == y && min_x <= original_x && original_x <= max_x
            });

        if let Some(watcher) = watcher {
            return watcher.direction == Direction::North;
        }
    }

    false
}
