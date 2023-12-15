use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let sequences = parse(input);

    let value = sequences
        .into_iter()
        .map(|sequence| {
            let mut current_value = 0;

            for char in sequence.chars() {
                current_value += char as u32;
                current_value *= 17;
                current_value %= 256;
            }

            current_value
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 1320);
    } else {
        println!("{}", value);
    }
}
