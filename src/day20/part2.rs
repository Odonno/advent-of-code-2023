use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let values = parse(input);

    println!("{:?}", values);

    let value = 0;

    if use_sample {
        assert_eq!(value, 0);
    } else {
        println!("{}", value);
    }
}
