use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::multi::many1;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    multi::separated_list1,
    IResult,
};

pub type SeedValue = u64;

#[derive(Debug, Clone)]
pub struct Input {
    pub seeds: Vec<SeedDetails>,
    pub categories: Vec<Category>,
}

#[derive(Debug, Clone)]
pub struct SeedDetails {
    value: SeedValue,
    length: u32,
}

#[derive(Debug, Clone)]
pub struct Category {
    pub name: String,
    pub ranges: Vec<CategoryRange>,
}

#[derive(Debug, Clone)]
pub struct CategoryRange {
    pub destination_start: SeedValue,
    pub source_start: SeedValue,
    pub length: SeedValue,
}

pub fn parse(input: &str, use_seed_length: bool) -> Input {
    parse_input(input, use_seed_length).unwrap().1
}

fn parse_input(input: &str, use_seed_length: bool) -> IResult<&str, Input> {
    let (input, _) = tag("seeds:")(input)?;
    let (input, _) = space1(input)?;
    let (input, seeds) = separated_list1(space1, digit1)(input)?;
    let seeds = match use_seed_length {
        true => {
            let seed_values = seeds
                .iter()
                .map(|v| v.parse::<SeedValue>().unwrap())
                .enumerate()
                .filter(|(index, _)| index % 2 == 0)
                .map(|(_, v)| v)
                .collect_vec();
            let seed_lengths = seeds
                .iter()
                .map(|v| v.parse::<u32>().unwrap())
                .enumerate()
                .filter(|(index, _)| index % 2 == 1)
                .map(|(_, v)| v)
                .collect_vec();

            seed_values
                .into_iter()
                .zip(seed_lengths)
                .map(|(value, length)| SeedDetails { value, length })
                .collect_vec()
        }
        false => seeds
            .iter()
            .map(|v| v.parse::<SeedValue>().unwrap())
            .map(|value| SeedDetails { value, length: 1 })
            .collect_vec(),
    };

    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;

    let (input, categories) = separated_list1(many1(line_ending), parse_category)(input)?;

    Ok((input, Input { seeds, categories }))
}

fn parse_category(input: &str) -> IResult<&str, Category> {
    let (input, name) = alt((
        tag("seed-to-soil"),
        tag("soil-to-fertilizer"),
        tag("fertilizer-to-water"),
        tag("water-to-light"),
        tag("light-to-temperature"),
        tag("temperature-to-humidity"),
        tag("humidity-to-location"),
    ))(input)?;
    let name = name.to_string();
    let (input, _) = space1(input)?;
    let (input, _) = tag("map")(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = line_ending(input)?;

    let (input, ranges) = separated_list1(line_ending, parse_category_range)(input)?;

    Ok((input, Category { name, ranges }))
}

fn parse_category_range(input: &str) -> IResult<&str, CategoryRange> {
    let (input, destination_start) = digit1(input)?;
    let destination_start = destination_start.parse::<SeedValue>().unwrap();
    let (input, _) = space1(input)?;

    let (input, source_start) = digit1(input)?;
    let source_start = source_start.parse::<SeedValue>().unwrap();
    let (input, _) = space1(input)?;

    let (input, length) = digit1(input)?;
    let length = length.parse::<SeedValue>().unwrap();

    Ok((
        input,
        CategoryRange {
            destination_start,
            source_start,
            length,
        },
    ))
}

pub fn get_min_location(seed_details: Vec<SeedDetails>, categories: Vec<Category>) -> SeedValue {
    let category1 = categories
        .iter()
        .find(|c| c.name == "seed-to-soil")
        .unwrap()
        .clone();
    let category2 = categories
        .iter()
        .find(|c| c.name == "soil-to-fertilizer")
        .unwrap()
        .clone();
    let category3 = categories
        .iter()
        .find(|c| c.name == "fertilizer-to-water")
        .unwrap()
        .clone();
    let category4 = categories
        .iter()
        .find(|c| c.name == "water-to-light")
        .unwrap()
        .clone();
    let category5 = categories
        .iter()
        .find(|c| c.name == "light-to-temperature")
        .unwrap()
        .clone();
    let category6 = categories
        .iter()
        .find(|c| c.name == "temperature-to-humidity")
        .unwrap()
        .clone();
    let category7 = categories
        .iter()
        .find(|c| c.name == "humidity-to-location")
        .unwrap()
        .clone();

    let locations = seed_details
        .into_iter()
        .map(|seed_detail| {
            let mut minimum = None;

            for seed in seed_detail.value..(seed_detail.value + seed_detail.length as u64) {
                let source = seed;
                let source = apply_category(&category1, source);
                let source = apply_category(&category2, source);
                let source = apply_category(&category3, source);
                let source = apply_category(&category4, source);
                let source = apply_category(&category5, source);
                let source = apply_category(&category6, source);
                let source = apply_category(&category7, source);

                if minimum.is_none() || source < minimum.unwrap() {
                    minimum = Some(source);
                }
            }

            minimum.unwrap()
        })
        .collect_vec();

    locations.iter().min().unwrap().clone()
}

pub fn apply_category(category: &Category, source: SeedValue) -> SeedValue {
    for range in category.ranges.iter() {
        let is_source_in_range =
            range.source_start <= source && source <= (range.source_start + range.length - 1);

        if is_source_in_range {
            let distance = source - range.source_start;
            return range.destination_start + distance;
        }
    }

    source
}
