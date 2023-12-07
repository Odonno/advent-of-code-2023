use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let hands = parse(input, false);
    let hands = sort_hands(hands);

    let value = calculate_value(hands);

    if use_sample {
        assert_eq!(value, 6440);
    } else {
        println!("{:?}", value);
    }
}
