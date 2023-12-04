use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

pub type CardId = u8;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: CardId,
    pub winning_numbers: Vec<u8>,
    pub card_numbers: Vec<u8>,
}

pub fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, id) = digit1(input)?;
    let id = id.parse::<u8>().unwrap();

    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, lists) = separated_list1(
        delimited(multispace1, char('|'), multispace1),
        separated_list1(multispace1, digit1),
    )(input)?;

    let winning_numbers = lists[0]
        .iter()
        .map(|n| n.parse::<u8>().unwrap())
        .collect_vec();

    let card_numbers = lists[1]
        .iter()
        .map(|n| n.parse::<u8>().unwrap())
        .collect_vec();

    Ok((
        input,
        Card {
            id,
            winning_numbers,
            card_numbers,
        },
    ))
}
