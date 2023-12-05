use std::collections::HashMap;

use super::common::*;

type CardCount = u32;
type CardCountMap = HashMap<CardId, CardCount>;

pub fn run(input: &str, use_sample: bool) {
    let original_cards = parse(input);
    let mut map = CardCountMap::new();

    for card in original_cards {
        let total_current_card_copies = map.get(&card.id).unwrap_or(&0);

        let total_current_card_count = total_current_card_copies + 1;
        map.insert(card.id, total_current_card_count);

        let total_valid_numbers = card
            .card_numbers
            .into_iter()
            .filter(|card_number| card.winning_numbers.contains(card_number))
            .collect::<Vec<_>>()
            .len();

        for id in (card.id + 1)..=(card.id + total_valid_numbers as u8) {
            let card_count = map.entry(id).or_insert(0);
            *card_count += total_current_card_count;
        }
    }

    let value = map.values().sum::<u32>();

    if use_sample {
        assert_eq!(value, 30);
    } else {
        println!("{:?}", value);
    }
}
