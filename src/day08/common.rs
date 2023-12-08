use nom::branch::alt;
use nom::character::complete::{alphanumeric1, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::{
    character::complete::{char, line_ending},
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct NodeInstructions {
    pub left: String,
    pub right: String,
}

pub type Node = String;
pub type Nodes = HashMap<Node, NodeInstructions>;

#[derive(Debug, Clone)]
pub struct Input {
    pub instructions: Vec<Instruction>,
    pub nodes: Nodes,
}

pub fn parse(input: &str) -> Input {
    parse_input(input).unwrap().1
}

fn parse_input(input: &str) -> IResult<&str, Input> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = line_ending(input)?;
    let (input, _) = line_ending(input)?;
    let (input, nodes) = parse_nodes(input)?;

    Ok((
        input,
        Input {
            instructions,
            nodes,
        },
    ))
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(alt((
        map(char('L'), |_| Instruction::Left),
        map(char('R'), |_| Instruction::Right),
    )))(input)?;

    Ok((input, instructions))
}

fn parse_nodes(input: &str) -> IResult<&str, Nodes> {
    let (input, nodes) = separated_list1(line_ending, parse_node)(input)?;
    let nodes = nodes
        .into_iter()
        .map(|(start, (left, right))| {
            (
                start.to_string(),
                NodeInstructions {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            )
        })
        .collect();

    Ok((input, nodes))
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, start) = alphanumeric1(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('(')(input)?;
    let (input, left) = alphanumeric1(input)?;
    let (input, _) = char(',')(input)?;
    let (input, _) = space1(input)?;
    let (input, right) = alphanumeric1(input)?;
    let (input, _) = char(')')(input)?;

    Ok((input, (start, (left, right))))
}
