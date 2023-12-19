use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, one_of},
    combinator::map,
    multi::separated_list1,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Part {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone)]
pub enum WorkflowResult {
    Accepted,
    Rejected,
    Workflow(String),
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub last: WorkflowResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleOperator {
    GreaterThan,
    LessThan,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub part: Part,
    pub operator: RuleOperator,
    pub value: u32,
    pub result: WorkflowResult,
}

pub type Rating = HashMap<Part, u32>;
pub type Workflows = HashMap<String, Workflow>;
pub type Ratings = Vec<Rating>;

#[derive(Debug, Clone)]
pub struct Input {
    pub workflows: Workflows,
    pub ratings: Ratings,
}

pub fn parse(input: &str) -> Input {
    let mut workflows = Workflows::new();
    let mut ratings = Ratings::new();

    for line in input.lines() {
        if let Ok((_, workflow)) = parse_workflow(line) {
            workflows.insert(workflow.name.to_string(), workflow);
        }
        if let Ok((_, rating)) = parse_rating(line) {
            ratings.push(rating);
        }
    }

    Input { workflows, ratings }
}

fn parse_rating(input: &str) -> IResult<&str, Rating> {
    let (input, _) = char('{')(input)?;
    let (input, values) = separated_list1(char(','), parse_rating_value)(input)?;
    let values = values.into_iter().collect::<Rating>();
    let (input, _) = char('}')(input)?;

    Ok((input, values))
}

fn parse_rating_value(input: &str) -> IResult<&str, (Part, u32)> {
    let (input, part) = parse_part(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = digit1(input)?;
    let value = value.parse::<u32>().unwrap();

    Ok((input, (part, value)))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    let name: String = name.to_string();

    let (input, _) = char('{')(input)?;
    let (input, rules) = separated_list1(char(','), parse_rule)(input)?;
    let (input, _) = char(',')(input)?;
    let (input, reject) = parse_workflow_result(input)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        Workflow {
            name,
            rules,
            last: reject,
        },
    ))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, part) = parse_part(input)?;
    let (input, operator) = parse_rule_operator(input)?;
    let (input, value) = digit1(input)?;
    let value = value.parse::<u32>().unwrap();

    let (input, _) = char(':')(input)?;
    let (input, result) = parse_workflow_result(input)?;

    Ok((
        input,
        Rule {
            part,
            operator,
            value,
            result,
        },
    ))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    alt((
        map(one_of("Xx"), |_| Part::X),
        map(one_of("Mm"), |_| Part::M),
        map(one_of("Aa"), |_| Part::A),
        map(one_of("Ss"), |_| Part::S),
    ))(input)
}

fn parse_workflow_name(input: &str) -> IResult<&str, String> {
    let (input, name) = alpha1(input)?;

    Ok((input, name.to_string()))
}

fn parse_rule_operator(input: &str) -> IResult<&str, RuleOperator> {
    alt((
        map(char('<'), |_| RuleOperator::LessThan),
        map(char('>'), |_| RuleOperator::GreaterThan),
    ))(input)
}

fn parse_workflow_result(input: &str) -> IResult<&str, WorkflowResult> {
    alt((
        map(char('A'), |_| WorkflowResult::Accepted),
        map(char('R'), |_| WorkflowResult::Rejected),
        map(parse_workflow_name, |name| WorkflowResult::Workflow(name)),
    ))(input)
}
