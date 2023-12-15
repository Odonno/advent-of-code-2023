use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let maps = parse(input);

    let reflections = maps
        .into_iter()
        .map(|map| find_reflections(&map).first().unwrap().clone())
        .collect_vec();

    let value = sum_reflections(reflections);

    if use_sample {
        assert_eq!(value, 405);
    } else {
        println!("{:?}", value);
    }
}
