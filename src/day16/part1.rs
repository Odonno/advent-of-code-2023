use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let entry_beam = Beam {
        position: (-1, 0),
        direction: Direction::Right,
    };

    let visited_beams = extract_visited_beams(&map, &entry_beam);
    let energized_positions = get_energized_positions(visited_beams);

    print_energized_map(&map, &energized_positions);

    let value = energized_positions.len();

    if use_sample {
        assert_eq!(value, 46);
    } else {
        println!("{}", value);
    }
}
