use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace1, one_of},
    combinator::map,
    IResult,
};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Hand {
    pub values: Vec<u8>,
    pub _type: HandType,
    pub bid: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

const JOKER_VALUE: u8 = 1;

pub fn parse(input: &str, use_joker: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|line| parse_line(line, use_joker).unwrap().1)
        .collect()
}

fn parse_line(input: &str, use_joker: bool) -> IResult<&str, Hand> {
    let (input, value1) = parse_card(input, use_joker)?;
    let (input, value2) = parse_card(input, use_joker)?;
    let (input, value3) = parse_card(input, use_joker)?;
    let (input, value4) = parse_card(input, use_joker)?;
    let (input, value5) = parse_card(input, use_joker)?;

    let values = vec![value1, value2, value3, value4, value5];

    let (input, _) = multispace1(input)?;

    let (input, bid) = digit1(input)?;
    let bid = bid.parse::<_>().unwrap();

    let _type = detect_hand_type(&values, use_joker);

    Ok((input, Hand { values, _type, bid }))
}

fn parse_card(input: &str, use_joker: bool) -> IResult<&str, u8> {
    alt((
        map(char('A'), |_| 14),
        map(char('K'), |_| 13),
        map(char('Q'), |_| 12),
        map(char('J'), |_| if use_joker { JOKER_VALUE } else { 11 }),
        map(char('T'), |_| 10),
        map(one_of("23456789"), |s| s.to_string().parse::<u8>().unwrap()),
    ))(input)
}

fn detect_hand_type(values: &Vec<u8>, use_joker: bool) -> HandType {
    let mut map: HashMap<&u8, u8> = HashMap::new();

    if use_joker {
        let number_of_jokers = values
            .iter()
            .filter(|v| v == &&JOKER_VALUE)
            .collect_vec()
            .len() as u8;

        for value in values {
            if value == &JOKER_VALUE {
                continue;
            }

            let entry = map.entry(value).or_insert(0u8);
            *entry += 1;
        }

        if number_of_jokers == values.len() as u8 {
            map.entry(&JOKER_VALUE).or_insert(values.len() as u8);
        }

        let has_joker = number_of_jokers > 0;
        if has_joker {
            let max_entry = map
                .iter()
                .sorted_by(|e1, e2| {
                    let order_one = Ord::cmp(e2.1, e1.1);

                    if order_one == Ordering::Equal {
                        Ord::cmp(e2.0, e1.0)
                    } else {
                        order_one
                    }
                })
                .nth(0)
                .unwrap();

            if max_entry.0 != &&JOKER_VALUE {
                let entry = map.entry(max_entry.0).or_insert(0u8);
                *entry += number_of_jokers;
            }
        }
    } else {
        for value in values {
            let entry = map.entry(value).or_insert(0u8);
            *entry += 1;
        }
    }

    let max_same_card = map.values().max().unwrap();

    match max_same_card {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            let has_pair = map.values().any(|v| v == &2);

            if has_pair {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            let pairs = map.values().filter(|v| v == &&2).collect_vec().len();

            match pairs {
                2 => HandType::TwoPairs,
                1 => HandType::OnePair,
                _ => panic!("Impossible"),
            }
        }
        1 => HandType::HighCard,
        _ => panic!("Impossible"),
    }
}

pub fn calculate_value(hands: Vec<Hand>) -> u32 {
    hands
        .into_iter()
        .enumerate()
        .map(|(index, card)| (index + 1) as u32 * card.bid)
        .sum::<u32>()
}

pub fn sort_hands(hands: Vec<Hand>) -> Vec<Hand> {
    hands
        .into_iter()
        .sorted_by(|a, b| {
            if a._type == b._type {
                for index in 0..a.values.len() {
                    if a.values[index] > b.values[index] {
                        return Ordering::Greater;
                    }
                    if a.values[index] < b.values[index] {
                        return Ordering::Less;
                    }
                }

                Ordering::Equal
            } else {
                if a._type < b._type {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
        })
        .collect_vec()
}
