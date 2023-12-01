use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let values = parse(input, false);
    let value = sum_calibration_values(values);

    if use_sample {
        assert_eq!(value, 142);
    } else {
        println!("Part 1: {}", value)
    }
}
