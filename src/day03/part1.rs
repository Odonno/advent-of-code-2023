use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let map = parse(input);

    let part_numbers_with_adjacent_symbol = get_part_numbers(&map)
        .into_iter()
        .filter(|(position, part)| {
            let (x1, y) = position.clone();
            let x2 = x1 + part.length - 1;

            let adjacent_positions = (x1.saturating_sub(1)..=x2 + 1)
                .flat_map(|adj_x| {
                    ((y.saturating_sub(1))..=(y + 1)).map(move |adj_y| (adj_x, adj_y))
                })
                .filter(|(adj_x, adj_y)| {
                    let is_num_part = adj_y == &y && (&x1 <= adj_x && adj_x <= &x2);
                    !is_num_part
                })
                .collect_vec();

            let adjacent_parts = adjacent_positions
                .into_iter()
                .filter_map(|pos| map.get(&pos))
                .collect_vec();

            let has_symbol = adjacent_parts.iter().any(|part| match part {
                EnginePart::Symbol(_) => true,
                _ => false,
            });

            has_symbol
        })
        .collect_vec();

    let value = part_numbers_with_adjacent_symbol
        .into_iter()
        .map(|n| n.1.value)
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 4361);
    } else {
        println!("{:?}", value);
    }
}
