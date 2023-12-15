use nom::character::complete::char;
use nom::{bytes::complete::take_while1, multi::separated_list1, IResult};

pub type Sequence = Vec<String>;

pub fn parse(input: &str) -> Sequence {
    parse_input(input)
        .unwrap()
        .1
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

fn parse_input(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(char(','), take_while1(|c| c != '\n' && c != ','))(input)
}
