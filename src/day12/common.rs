use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Row {
    pub springs: Vec<Spring>,
    pub contiguous_group_of_damaged_springs: Vec<u8>,
}

pub fn parse(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Row> {
    let (input, springs) = many1(parse_spring)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, contiguous_group_of_damaged_springs) = separated_list1(char(','), digit1)(input)?;
    let contiguous_group_of_damaged_springs = contiguous_group_of_damaged_springs
        .into_iter()
        .map(|c| c.parse::<u8>().unwrap())
        .collect_vec();

    Ok((
        input,
        Row {
            springs,
            contiguous_group_of_damaged_springs,
        },
    ))
}

fn parse_spring(input: &str) -> IResult<&str, Spring> {
    alt((
        map(tag("."), |_| Spring::Operational),
        map(tag("#"), |_| Spring::Damaged),
        map(tag("?"), |_| Spring::Unknown),
    ))(input)
}
