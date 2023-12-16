use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let max_x = map.keys().map(|(x, _)| x).max().unwrap().clone();
    let max_y = map.keys().map(|(_, y)| y).max().unwrap().clone();

    let mut entry_beams = Vec::new();

    for y in 0..=max_y {
        entry_beams.push(Beam {
            position: (-1, y),
            direction: Direction::Right,
        });
        entry_beams.push(Beam {
            position: (max_x + 1, y),
            direction: Direction::Left,
        });
    }

    for x in 0..=max_x {
        entry_beams.push(Beam {
            position: (x, -1),
            direction: Direction::Bottom,
        });
        entry_beams.push(Beam {
            position: (x, max_y + 1),
            direction: Direction::Top,
        });
    }

    let value = entry_beams
        .iter()
        .map(|entry_beam| {
            let visited_beams = extract_visited_beams(&map, entry_beam);
            let energized_positions = get_energized_positions(visited_beams);

            energized_positions.len() as u32
        })
        .max()
        .unwrap();

    if use_sample {
        assert_eq!(value, 51);
    } else {
        println!("{}", value);
    }
}
