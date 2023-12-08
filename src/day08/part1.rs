use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);
    let value = count_steps_to_end(&input, "AAA", vec!["ZZZ"]);

    if use_sample {
        assert_eq!(value, 6);
    } else {
        println!("{:?}", value);
    }
}
