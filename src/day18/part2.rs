use super::common::*;

type Position = (i64, i64);
type Positions = Vec<Position>;

pub fn run(input: &str, use_sample: bool) {
    let instructions = parse(input, true);

    let initial_position: Position = (0, 0);

    let mut current_position: Position = initial_position;
    let mut vertices = Positions::new();
    let mut boundary = 1;

    vertices.push(current_position);

    for instruction in instructions {
        let Instruction {
            direction, meters, ..
        } = instruction;

        let (x, y) = current_position;
        let meters = meters as i64;

        current_position = match direction {
            Direction::Right => (x + meters, y),
            Direction::Left => (x - meters, y),
            Direction::Up => (x, y - meters),
            Direction::Down => (x, y + meters),
        };

        vertices.push(current_position);
        boundary += meters;
    }

    let area = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>();
    let area = area.abs() / 2;

    let value = area + boundary / 2 + 1;

    if use_sample {
        assert_eq!(value, 952408144115);
    } else {
        println!("{}", value);
    }
}
