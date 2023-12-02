use super::common::*;

const BAG: Bag = Bag {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn run(input: &str, use_sample: bool) {
    let rounds = parse(input);

    let value = rounds
        .iter()
        .filter(|round| {
            let is_possible = round.sets.iter().all(|set| {
                set.red.unwrap_or(0) <= BAG.red
                    && set.green.unwrap_or(0) <= BAG.green
                    && set.blue.unwrap_or(0) <= BAG.blue
            });

            is_possible
        })
        .map(|round| round.id as u32)
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 8);
    } else {
        println!("{:?}", value);
    }
}
