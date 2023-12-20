use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, space1},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Button,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub _type: ModuleType,
}

#[derive(Debug, Clone)]
pub struct ModuleConfiguration {
    pub module: Module,
    pub destinations: Vec<String>,
}

pub type PuzzleConfiguration = Vec<ModuleConfiguration>;

pub fn parse(input: &str) -> PuzzleConfiguration {
    input
        .lines()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, ModuleConfiguration> {
    let (input, module) = parse_module(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space1(input)?;
    let (input, destinations) = separated_list1(tag(", "), alpha1)(input)?;
    let destinations = destinations.into_iter().map(|s| s.to_string()).collect();

    Ok((
        input,
        ModuleConfiguration {
            module,
            destinations,
        },
    ))
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    alt((
        map(preceded(tag("&"), alpha1), |name: &str| Module {
            name: name.to_string(),
            _type: ModuleType::Conjunction,
        }),
        map(preceded(tag("%"), alpha1), |name: &str| Module {
            name: name.to_string(),
            _type: ModuleType::FlipFlop,
        }),
        map(tag("broadcaster"), |_| Module {
            name: "broadcaster".to_string(),
            _type: ModuleType::Broadcaster,
        }),
    ))(input)
}
