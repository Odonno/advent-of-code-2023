use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let Input { seeds, categories } = parse(input, true);

    let value = get_min_location(seeds, categories);

    if use_sample {
        assert_eq!(value, 46);
    } else {
        println!("{:?}", value);
    }
}
