use itertools::Itertools;
use std::collections::HashSet;

use super::common::*;

type Position = (i32, i32);
type Positions = HashSet<Position>;

pub fn run(input: &str, use_sample: bool) {
    let instructions = parse(input, false);

    let initial_position: Position = (0, 0);

    let mut current_position: Position = initial_position;
    let mut edge_points = Positions::new();

    edge_points.insert(current_position);

    for instruction in instructions {
        let Instruction {
            direction, meters, ..
        } = instruction;

        for _ in 0..meters {
            let (x, y) = current_position;

            current_position = match direction {
                Direction::Right => (x + 1, y),
                Direction::Left => (x - 1, y),
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
            };

            edge_points.insert(current_position);
        }
    }

    let mut visited_interiors = Positions::new();
    let mut to_visit_interiors_points = Positions::new();

    to_visit_interiors_points.insert((1, 1));

    while !to_visit_interiors_points.is_empty() {
        let points_to_visit = to_visit_interiors_points.drain().collect_vec();

        for point in points_to_visit.clone() {
            if edge_points.contains(&point) {
                continue;
            }
            if visited_interiors.contains(&point) {
                continue;
            }

            let (x, y) = point;

            let left = (x - 1, y);
            let right = (x + 1, y);
            let top = (x, y - 1);
            let down = (x, y + 1);

            if !points_to_visit.contains(&left) {
                to_visit_interiors_points.insert(left);
            }
            if !points_to_visit.contains(&right) {
                to_visit_interiors_points.insert(right);
            }
            if !points_to_visit.contains(&top) {
                to_visit_interiors_points.insert(top);
            }
            if !points_to_visit.contains(&down) {
                to_visit_interiors_points.insert(down);
            }

            visited_interiors.insert(point);
        }
    }

    let value = edge_points.len() + visited_interiors.len();

    if use_sample {
        assert_eq!(value, 62);
    } else {
        println!("{}", value);
    }
}
