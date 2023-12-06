use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let race = parse_part2(input);

    let value = get_total_ways(vec![race]);

    if use_sample {
        assert_eq!(value, 71503);
    } else {
        println!("{:?}", value);
    }
}
