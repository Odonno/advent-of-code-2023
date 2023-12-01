use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let values = parse(input, true);
    let value = sum_calibration_values(values);

    if use_sample {
        assert_eq!(value, 281);
    } else {
        println!("Part 2: {}", value)
    }
}
