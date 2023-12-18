use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, multispace1},
    combinator::map,
    sequence::delimited,
    IResult,
};

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub direction: Direction,
    pub meters: u32,
}

pub fn parse(input: &str, from_color: bool) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| parse_line(line, from_color).unwrap().1)
        .collect()
}

fn parse_line(input: &str, from_color: bool) -> IResult<&str, Instruction> {
    let (input, direction) = parse_direction(input)?;
    let (input, _) = multispace1(input)?;

    let (input, meters) = digit1(input)?;
    let meters = meters.parse::<_>().unwrap();

    let (input, _) = multispace1(input)?;

    let (input, color) = delimited(tag("(#"), alphanumeric1, char(')'))(input)?;

    let instruction = if from_color {
        let meters = u32::from_str_radix(&color[0..5], 16).unwrap();

        let direction = match color.chars().nth(5).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Unknown direction"),
        };

        Instruction { direction, meters }
    } else {
        Instruction { direction, meters }
    };

    Ok((input, instruction))
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        map(char('U'), |_| Direction::Up),
        map(char('R'), |_| Direction::Right),
        map(char('L'), |_| Direction::Left),
        map(char('D'), |_| Direction::Down),
    ))(input)
}
