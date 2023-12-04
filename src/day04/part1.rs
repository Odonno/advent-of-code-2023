use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let cards = parse(input);

    let value = cards
        .iter()
        .map(|card| {
            let valid_numbers = card
                .card_numbers
                .iter()
                .filter(|card_number| card.winning_numbers.contains(card_number))
                .collect::<Vec<_>>();

            if valid_numbers.len() == 0 {
                0
            } else {
                u32::pow(2, (valid_numbers.len() - 1) as u32)
            }
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 13);
    } else {
        println!("{:?}", value);
    }
}
