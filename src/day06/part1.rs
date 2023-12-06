use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let races = parse(input);

    let value = get_total_ways(races);

    if use_sample {
        assert_eq!(value, 288);
    } else {
        println!("{:?}", value);
    }
}
