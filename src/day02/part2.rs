use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let rounds = parse(input);

    let value = rounds
        .iter()
        .map(|round| {
            let max_red = round
                .sets
                .iter()
                .map(|set| set.red.unwrap_or(0))
                .max()
                .unwrap() as u32;

            let max_blue = round
                .sets
                .iter()
                .map(|set| set.blue.unwrap_or(0))
                .max()
                .unwrap() as u32;

            let max_green = round
                .sets
                .iter()
                .map(|set| set.green.unwrap_or(0))
                .max()
                .unwrap() as u32;

            max_red * max_blue * max_green
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 2286);
    } else {
        println!("{:?}", value);
    }
}
