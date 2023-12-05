use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let Input { seeds, categories } = parse(input, false);

    let value = get_min_location(seeds, categories);

    if use_sample {
        assert_eq!(value, 35);
    } else {
        println!("{:?}", value);
    }
}
