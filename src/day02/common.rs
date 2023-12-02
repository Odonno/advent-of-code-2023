use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace1},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Bag {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone)]
pub struct GameRound {
    pub id: u8,
    pub sets: Vec<CubeSet>,
}

#[derive(Debug, Clone)]
pub struct CubeSet {
    pub red: Option<u8>,
    pub green: Option<u8>,
    pub blue: Option<u8>,
}

#[derive(Debug, Clone)]
struct ParsedCubeReveal {
    count: u8,
    color: String,
}

pub fn parse(input: &str) -> Vec<GameRound> {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, GameRound> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = digit1(input)?;
    let id = id.parse::<u8>().unwrap();

    let (input, _) = tag(":")(input)?;

    let parsed_reveals =
        separated_list1(char(';'), separated_list1(char(','), parse_cube_reveal))(input)?;

    let sets = parsed_reveals
        .1
        .iter()
        .map(|reveals| CubeSet {
            red: reveals.iter().find(|r| r.color == "red").map(|r| r.count),
            green: reveals.iter().find(|r| r.color == "green").map(|r| r.count),
            blue: reveals.iter().find(|r| r.color == "blue").map(|r| r.count),
        })
        .collect_vec();

    Ok((input, GameRound { id, sets }))
}

fn parse_cube_reveal(input: &str) -> IResult<&str, ParsedCubeReveal> {
    let (input, _) = multispace1(input)?;
    let (input, count) = digit1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, color) = nom::character::complete::alphanumeric1(input)?;

    Ok((
        input,
        ParsedCubeReveal {
            count: count.parse().unwrap(),
            color: color.to_string(),
        },
    ))
}
