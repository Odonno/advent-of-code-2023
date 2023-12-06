use itertools::Itertools;
use nom::character::complete::line_ending;
use nom::character::complete::multispace1;
use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult};

#[derive(Debug, Clone)]
pub struct Race {
    pub time: u32,
    pub distance: u64,
}

pub fn parse(input: &str) -> Vec<Race> {
    parse_input(input).unwrap().1
}

fn parse_input(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, times) = separated_list1(multispace1, digit1)(input)?;
    let times = times
        .into_iter()
        .map(|v| v.parse::<_>().unwrap())
        .collect_vec();

    let (input, _) = line_ending(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distances) = separated_list1(multispace1, digit1)(input)?;
    let distances = distances
        .into_iter()
        .map(|v| v.parse::<_>().unwrap())
        .collect_vec();

    let races = times
        .into_iter()
        .zip(distances)
        .into_iter()
        .map(|(time, distance)| Race { time, distance })
        .collect_vec();

    Ok((input, races))
}

pub fn parse_part2(input: &str) -> Race {
    parse_input_part2(input).unwrap().1
}

fn parse_input_part2(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, times) = separated_list1(multispace1, digit1)(input)?;
    let time = times.concat();
    let time = time.parse::<_>().unwrap();

    let (input, _) = line_ending(input)?;

    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distances) = separated_list1(multispace1, digit1)(input)?;
    let distance = distances.concat();
    let distance = distance.parse::<_>().unwrap();

    Ok((input, Race { time, distance }))
}

pub fn get_total_ways(races: Vec<Race>) -> u64 {
    races.into_iter().map(calculate_ways).product::<_>()
}

fn calculate_ways(race: Race) -> u64 {
    let delta = u64::pow(race.time as u64, 2) - (4 * race.distance);
    let delta = (delta as f64).sqrt();

    let x1 = (-(race.time as f64) + delta) / -2f64;
    let x2 = (-(race.time as f64) - delta) / -2f64;

    (x2.ceil() - x1.floor()) as u64 - 1
}
