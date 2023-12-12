use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let rows = parse(input);

    let value = rows
        .into_iter()
        .map(|row| count_arrangements(row))
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 21);
    } else {
        println!("{:?}", value);
    }
}

fn count_arrangements(row: Row) -> u32 {
    let current_damaged = row
        .springs
        .clone()
        .into_iter()
        .filter(|s| s == &Spring::Damaged)
        .collect_vec()
        .len() as u8;
    let expected_total_damaged = row.contiguous_group_of_damaged_springs.iter().sum::<u8>();

    let damaged_to_add = expected_total_damaged - current_damaged;

    let unknown_indexes = row
        .springs
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, s)| s == &Spring::Unknown)
        .map(|(index, _)| index)
        .collect_vec();

    let permutations = unknown_indexes
        .into_iter()
        .combinations(damaged_to_add.into())
        .collect_vec();

    let all_arrangements = permutations
        .into_iter()
        .map(|damages_indexes| {
            let mut new_springs = row.springs.clone();

            for index in damages_indexes.into_iter() {
                new_springs[index] = Spring::Damaged;
            }

            new_springs
        })
        .collect_vec();

    let valid_arrangements = all_arrangements
        .into_iter()
        .filter(|springs| is_valid_arrangement(springs, &row.contiguous_group_of_damaged_springs))
        .collect_vec();

    valid_arrangements.len() as u32
}

fn is_valid_arrangement(
    springs: &Vec<Spring>,
    contiguous_group_of_damaged_springs: &Vec<u8>,
) -> bool {
    let groups = springs
        .into_iter()
        .dedup_by_with_count(|a, b| a == b)
        .filter(|(_, s)| s == &&Spring::Damaged)
        .map(|(x, _)| x as u8)
        .collect_vec();

    contiguous_group_of_damaged_springs == &groups
}
