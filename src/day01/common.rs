use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, combinator::map_res, IResult,
};

#[derive(Debug, Clone, Copy)]
pub struct CalibrationValue {
    pub first_digit: u8,
    pub last_digit: u8,
}

type ParsedDigit = u8;

pub fn parse(input: &str, with_spelled_letters: bool) -> Vec<CalibrationValue> {
    input
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .enumerate()
                .filter_map(|(index, _)| {
                    let text = &line[index..];

                    match parse_digit(text, with_spelled_letters) {
                        Ok((_, digit)) => Some(digit),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>();

            let first_digit = digits.first().unwrap().clone();
            let last_digit = digits.last().unwrap().clone();

            CalibrationValue {
                first_digit,
                last_digit,
            }
        })
        .collect()
}

fn parse_digit(input: &str, with_spelled_letters: bool) -> IResult<&str, ParsedDigit> {
    let mut single_digit_parse = map_res(one_of("123456789"), |s| {
        Ok::<_, ()>(s.to_digit(10).unwrap() as u8)
    });

    if with_spelled_letters {
        alt((
            single_digit_parse,
            map_res(tag("one"), |_| Ok::<_, ()>(1)),
            map_res(tag("two"), |_| Ok::<_, ()>(2)),
            map_res(tag("three"), |_| Ok::<_, ()>(3)),
            map_res(tag("four"), |_| Ok::<_, ()>(4)),
            map_res(tag("five"), |_| Ok::<_, ()>(5)),
            map_res(tag("six"), |_| Ok::<_, ()>(6)),
            map_res(tag("seven"), |_| Ok::<_, ()>(7)),
            map_res(tag("eight"), |_| Ok::<_, ()>(8)),
            map_res(tag("nine"), |_| Ok::<_, ()>(9)),
        ))(input)
    } else {
        single_digit_parse(input)
    }
}

pub fn sum_calibration_values(values: Vec<CalibrationValue>) -> i32 {
    values
        .iter()
        .map(|v| (v.first_digit * 10 + v.last_digit) as i32)
        .sum::<i32>()
}
