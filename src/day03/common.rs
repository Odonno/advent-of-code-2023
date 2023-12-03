use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, digit1},
    combinator::map,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnginePart {
    Period,
    Symbol(char),
    Number(EnginePartNumber),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnginePartNumber {
    pub value: u32,
    pub length: u8,
}

pub type Position = (u8, u8);
pub type EnginePartMap = HashMap<Position, EnginePart>;

pub fn parse(input: &str) -> EnginePartMap {
    let mut map = EnginePartMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;
        let mut input = line;

        while let Ok((remaining, engine_part)) = parse_line(input) {
            map.insert((x, y as u8), engine_part);

            match engine_part {
                EnginePart::Number(part) => x += part.length as u8,
                _ => x += 1,
            }
            input = remaining;
        }
    });

    map
}

fn parse_line(input: &str) -> IResult<&str, EnginePart> {
    let (input, part) = alt((
        map(tag("."), |_| EnginePart::Period),
        map(digit1, |n: &str| {
            EnginePart::Number(EnginePartNumber {
                value: n.parse::<u32>().unwrap(),
                length: n.len() as u8,
            })
        }),
        map(anychar, |c| EnginePart::Symbol(c)),
    ))(input)?;

    Ok((input, part))
}

pub fn get_part_numbers(map: &EnginePartMap) -> Vec<(Position, EnginePartNumber)> {
    map.clone()
        .into_iter()
        .filter_map(|(position, part)| match part {
            EnginePart::Number(part) => Some((position, part)),
            _ => None,
        })
        .collect_vec()
}
