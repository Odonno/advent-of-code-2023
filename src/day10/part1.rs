use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);

    let correct_path = find_correct_path(&input);

    let steps = correct_path.len();
    let value = steps / 2;

    if use_sample {
        assert_eq!(value, 8);
    } else {
        println!("{:?}", value);
    }
}
