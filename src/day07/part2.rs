use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let hands = parse(input, true);
    let hands = sort_hands(hands);

    let value = calculate_value(hands);

    if use_sample {
        assert_eq!(value, 5905);
    } else {
        println!("{:?}", value);
    }
}
