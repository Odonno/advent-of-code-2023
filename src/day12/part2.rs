use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use super::common::*;

type DamagedLength = u8;
type CombinationCount = usize;

// type ContiguousCombination = Vec<(DamagedLength, CombinationCount)>;
// type DamagedLength = u8;
// type CombinationCount = u8;

type PatternCombination = Vec<Spring>;
type ExpectedCombination = Vec<DamagedLength>;

type ContiguousCombinations = Vec<(CombinationCount, ExpectedCombination)>;

type Patterns = HashMap<PatternCombination, ContiguousCombinations>;

pub fn run(input: &str, use_sample: bool) {
    let rows = parse(input);

    //let mut x = HashMap::new();

    // {
    //     x.entry(vec![Spring::Operational]).or_insert(1);
    //     x.entry(vec![Spring::Damaged]).or_insert(1);
    // }

    // {
    //     x.entry(vec![Spring::Operational])
    //         .and_modify(|x| *x = &*x + 1);
    // }

    //println!("{:?}", x);
    // let rows = rows
    //     .into_iter()
    //     .map(|row| {
    //         let springs = vec![
    //             row.springs.clone(),
    //             row.springs.clone(),
    //             row.springs.clone(),
    //             row.springs.clone(),
    //             row.springs.clone(),
    //         ]
    //         .join(&Spring::Unknown);
    //         let contiguous_group_of_damaged_springs =
    //             row.contiguous_group_of_damaged_springs.repeat(5);

    //         Row {
    //             springs,
    //             contiguous_group_of_damaged_springs,
    //         }
    //     })
    //     .collect_vec();

    //println!("{:?}", rows);

    let mut patterns = Patterns::new();

    let value = rows
        .into_iter()
        .map(|row| {
            let x = count_arrangements(row, &mut patterns);
            println!("count_arrangements {:?}", x);
            x
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 525152);
    } else {
        println!("{:?}", value);
    }
}

fn count_arrangements(row: Row, _: &mut Patterns) -> u32 {
    let mut patterns = Patterns::new();
    //println!("count_arrangements");

    let springs = vec![
        row.springs.clone(),
        row.springs.clone(),
        row.springs.clone(),
        row.springs.clone(),
        row.springs.clone(),
    ]
    .join(&Spring::Unknown);

    //println!("{:?}", springs);

    let contiguous_group_of_damaged_springs = row.contiguous_group_of_damaged_springs.repeat(5);

    let possible_damaged_lengths = row
        .contiguous_group_of_damaged_springs
        .clone()
        .into_iter()
        .collect::<HashSet<u8>>();

    let max_contiguous_group_of_damaged_springs =
        possible_damaged_lengths.clone().into_iter().max().unwrap();

    //println!("{:?}", possible_damaged_lengths);
    // .into_iter()
    // .collect();

    //println!("{:?}", springs);

    // cut each contiguous blocks and get/store them in a hashmap
    let contiguous_blocks_indexes = springs
        .iter()
        .enumerate()
        .dedup_by(|a, b| {
            if a.1 == &Spring::Operational {
                b.1 == &Spring::Operational
            } else {
                b.1 != &Spring::Operational
            }
        })
        .map(|(index, _)| index)
        // .dedup_by_with_count(|a, b| {
        //     if a.1 == Spring::Operational {
        //         b.1 == Spring::Operational
        //     } else {
        //         b.1 != Spring::Operational
        //     }
        // })
        // .map(||)
        .chain(vec![springs.len()])
        .collect_vec();

    //println!("{:?}", contiguous_blocks_indexes);

    let mut current_row_patterns = Vec::new();

    for (start, end) in contiguous_blocks_indexes.into_iter().tuple_windows() {
        let contiguous_block = &springs[start..end];

        let is_ok = contiguous_block.iter().all(|s| s != &Spring::Operational);
        if !is_ok {
            continue;
        }

        let len = end - start - 1;
        println!("len {:?}", len);
        println!(
            "max_contiguous_group_of_damaged_springs {:?}",
            max_contiguous_group_of_damaged_springs
        );

        if len > max_contiguous_group_of_damaged_springs.into() {
            let modulo = num::integer::gcd(len, max_contiguous_group_of_damaged_springs as usize);
            // let modulo = if modulo < max_contiguous_group_of_damaged_springs.into() {
            //     max_contiguous_group_of_damaged_springs.into()
            // } else {
            //     modulo
            // };
            println!("modulo {:?}", modulo);

            // cut range into smaller ranges
            let mut temp_start = start;
            let mut temp_end = start + modulo as usize;

            loop {
                let contiguous_block = &springs[temp_start..temp_end];
                //println!("cb1 {:?}", contiguous_block);

                let current_block_patterns =
                    patterns.entry(contiguous_block.into()).or_insert_with(|| {
                        get_contiguous_combinations(
                            contiguous_block,
                            &possible_damaged_lengths,
                            &max_contiguous_group_of_damaged_springs,
                        )
                    });

                let current_block_patterns = current_block_patterns.clone();
                current_row_patterns.push(current_block_patterns);

                temp_start += modulo;
                temp_end += modulo;

                if temp_start > end {
                    break;
                }
                if temp_end > end {
                    temp_end = end;
                }
            }

            continue;
        }

        //println!("{:?}", contiguous_block);
        //println!("not all operationals");
        //println!("cb2 {:?}", contiguous_block);

        let current_block_patterns = patterns.entry(contiguous_block.into()).or_insert_with(|| {
            get_contiguous_combinations(
                contiguous_block,
                &possible_damaged_lengths,
                &max_contiguous_group_of_damaged_springs,
            )
        });

        let current_block_patterns = current_block_patterns.clone();
        current_row_patterns.push(current_block_patterns);

        // if current_block_patterns.len() > max_contiguous_group_of_damaged_springs.into() {
        //     for x in current_block_patterns.chunks(max_contiguous_group_of_damaged_springs.into()) {
        //         current_row_patterns.push(x.to_vec());
        //     }
        // } else {

        // }

        //current_row_patterns.push(current_block_patterns);

        // if current_block_patterns.len() > max_contiguous_group_of_damaged_springs.into() {
        //     current_row_patterns.push(vec![current_block_patterns]);
        // } else {
        //     current_row_patterns.push(vec![current_block_patterns]);
        // }
    }

    // iterate over vec of contiguous blocks to all possible arrangements
    let current_row_patterns = current_row_patterns;

    println!("{:?}", current_row_patterns);

    // let possible_contiguous_group_of_damaged_springs = current_row_patterns
    //     .iter()
    //     .multi_cartesian_product()
    //     .collect_vec();

    let valid_arrangements = current_row_patterns
        .into_iter()
        .multi_cartesian_product()
        .map(|ps| {
            // let mut global_index = 0;
            // println!("{:?}", ps);

            // ps.iter()
            //     .map(|p| {
            //         let mut total_count = 1;
            //         let mut index = global_index;

            //         for (count, expected_combination) in p {
            //             for value in expected_combination {
            //                 if value != &contiguous_group_of_damaged_springs[index] {
            //                     return 0;
            //                 }
            //                 index += 1;
            //             }

            //             total_count *= count;
            //         }

            //         println!("{:?}", total_count);
            //         total_count as u32
            //     })
            //     .sum::<u32>()

            let mut total_count = 1;
            let mut index = 0;

            //let mut blocks_iter = contiguous_group_of_damaged_springs.iter();

            for (count, expected_combination) in ps {
                for value in expected_combination {
                    // if &value != blocks_iter.next().unwrap() {
                    //     return 0;
                    // }
                    if &value != &contiguous_group_of_damaged_springs[index] {
                        return 0;
                    }
                    index += 1;
                }

                total_count *= count;
            }

            println!("{:?}", total_count);
            //println!("{:?}", p.clone());
            total_count as u32
            //println!("{:?}", p);
            // TODO
        });
    //.collect_vec();

    //println!("{:?}", possible_contiguous_group_of_damaged_springs);

    //let possible_contiguous_group_of_damaged_springs = Vec::new();

    // for row_pattern in current_row_patterns {
    //     for combination in row_pattern {
    //         let (count, expected_combination) = combination;

    //         //println!("count {:?} expected_combination {:?}", count, expected_combination);
    //     }
    //     //println!("{:?}", row_pattern);
    // }
    //println!("{:?}", current_row_patterns);

    //let contiguous_blocks
    // let valid_paths = possible_contiguous_group_of_damaged_springs
    //     .iter()
    //     .filter(|(count, p)| p == &&contiguous_group_of_damaged_springs)
    //     .collect_vec();

    // valid_paths.iter().map(|(count, _)| count).sum::<u32>() as u32

    valid_arrangements.sum::<u32>()
    //0
}

fn get_contiguous_combinations(
    contiguous_block: &[Spring],
    possible_damaged_lengths: &HashSet<u8>,
    max_contiguous_group_of_damaged_springs: &u8,
) -> ContiguousCombinations {
    //let mut x = ContiguousCombinations::new();

    let combinations = contiguous_block
        .into_iter()
        .map(|s| {
            if s == &Spring::Damaged {
                vec![s.clone()]
            } else {
                vec![Spring::Damaged, Spring::Operational]
            }
        })
        .multi_cartesian_product()
        .filter_map(|combination| {
            //println!("c");
            let number_of_damages = combination
                .iter()
                .filter(|spring| spring == &&Spring::Damaged)
                .collect_vec()
                .len() as u8;

            let is_possible = &number_of_damages <= max_contiguous_group_of_damaged_springs;
            if !is_possible {
                return None;
            }

            let c = combination
                .iter()
                .dedup_with_count()
                .filter(|(_, spring)| spring == &&Spring::Damaged)
                .map(|(count, _)| count as u8)
                //.filter(|count| count == &0 || possible_damaged_lengths.contains(count))
                .collect_vec();
            //println!("combination {:?}", c);

            let is_possible = c
                .iter()
                .all(|count| count == &0 || possible_damaged_lengths.contains(count));

            if is_possible {
                Some(c)
            } else {
                None
            }
        })
        .dedup_with_count()
        .collect_vec();

    println!("cc {:?}", combinations);

    combinations

    //x
}

// fn is_valid_arrangement(
//     springs: &Vec<Spring>,
//     contiguous_group_of_damaged_springs: &Vec<u8>,
// ) -> bool {
//     true
//     // let groups = springs
//     //     .into_iter()
//     //     .dedup_by_with_count(|a, b| a == b)
//     //     .filter(|(_, s)| s == &&Spring::Damaged)
//     //     .map(|(x, _)| x as u8)
//     //     .collect_vec();

//     // contiguous_group_of_damaged_springs == &groups
//     groups == contiguous_group_of_damaged_springs.clone()
// }

// fn is_valid_arrangement(
//     springs: &Vec<Spring>,
//     contiguous_group_of_damaged_springs: &Vec<u8>,
// ) -> bool {
//     true
//     // let groups = springs
//     //     .into_iter()
//     //     .dedup_by_with_count(|a, b| a == b)
//     //     .filter(|(_, s)| s == &&Spring::Damaged)
//     //     .map(|(x, _)| x as u8)
//     //     .collect_vec();

//     // contiguous_group_of_damaged_springs == &groups
//     groups == contiguous_group_of_damaged_springs.clone()
// }

//.dedup_by_with_count(|a, b| a == b)
//.filter(|(_, s)| s == &&Spring::Damaged)
//.map(|(x, _)| x as u8)

// let current_damaged = springs
//     .iter()
//     .filter(|s| s == &&Spring::Damaged)
//     .collect_vec()
//     .len() as u8;
// // let current_damaged = springs
// //     .clone()
// //     .into_iter()
// //     .filter(|s| s == &Spring::Damaged)
// //     .collect_vec()
// //     .len() as u8;
// let expected_total_damaged = contiguous_group_of_damaged_springs.iter().sum::<u8>();

// let damaged_to_add = expected_total_damaged - current_damaged;

// let unknown_indexes = springs
//     .iter()
//     .enumerate()
//     .filter(|(_, s)| s == &&Spring::Unknown)
//     .map(|(index, _)| index)
//     .collect_vec();
// // let unknown_indexes = springs
// //     .clone()
// //     .into_iter()
// //     .enumerate()
// //     .filter(|(_, s)| s == &Spring::Unknown)
// //     .map(|(index, _)| index)
// //     .collect_vec();

// let permutations = unknown_indexes
//     .into_iter()
//     .combinations(damaged_to_add.into())
//     .collect_vec();

// let all_arrangements = permutations
//     .into_iter()
//     .map(|damages_indexes| {
//         let mut new_springs = springs.clone();

//         for index in damages_indexes.into_iter() {
//             new_springs[index] = Spring::Damaged;
//         }

//         new_springs
//     })
//     .collect_vec();

// let valid_arrangements = all_arrangements
//     .into_iter()
//     .filter(|springs| is_valid_arrangement(springs, &contiguous_group_of_damaged_springs))
//     .collect_vec();

// valid_arrangements.len() as u32
