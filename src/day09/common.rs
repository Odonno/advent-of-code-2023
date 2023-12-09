use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace1},
    combinator::{opt, recognize},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

pub type History = Vec<i32>;

pub fn parse(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, History> {
    let (input, values) =
        separated_list1(multispace1, recognize(preceded(opt(tag("-")), digit1)))(input)?;
    let values = values
        .iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect_vec();

    Ok((input, values))
}
