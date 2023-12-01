use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, combinator::map, IResult,
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
            let line_len = line.len();

            let mut first_digit: u8 = 0;

            for index in 0..line_len {
                let text = &line[index..];

                match parse_digit(text, with_spelled_letters) {
                    Some(digit) => {
                        first_digit = digit;
                        break;
                    }
                    None => continue,
                }
            }

            let mut last_digit: u8 = 0;

            for index in (0..line_len).rev() {
                let text = &line[index..];

                match parse_digit(text, with_spelled_letters) {
                    Some(digit) => {
                        last_digit = digit;
                        break;
                    }
                    None => continue,
                }
            }

            CalibrationValue {
                first_digit,
                last_digit,
            }
        })
        .collect()
}

fn parse_digit(input: &str, with_spelled_letters: bool) -> Option<ParsedDigit> {
    const USE_NOM: bool = false;

    if USE_NOM {
        match parse_digit_nom(input, with_spelled_letters) {
            Ok((_, digit)) => Some(digit),
            Err(_) => None,
        }
    } else {
        parse_digit_std(input, with_spelled_letters)
    }
}

fn parse_digit_std(input: &str, with_spelled_letters: bool) -> Option<ParsedDigit> {
    let c = input.chars().next().unwrap();

    if c.is_digit(10) {
        Some(c.to_digit(10).unwrap() as u8)
    } else {
        if with_spelled_letters {
            if input.starts_with("one") {
                Some(1)
            } else if input.starts_with("two") {
                Some(2)
            } else if input.starts_with("three") {
                Some(3)
            } else if input.starts_with("four") {
                Some(4)
            } else if input.starts_with("five") {
                Some(5)
            } else if input.starts_with("six") {
                Some(6)
            } else if input.starts_with("seven") {
                Some(7)
            } else if input.starts_with("eight") {
                Some(8)
            } else if input.starts_with("nine") {
                Some(9)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn parse_digit_nom(input: &str, with_spelled_letters: bool) -> IResult<&str, ParsedDigit> {
    let mut single_digit_parse = map(one_of("123456789"), |s| s.to_digit(10).unwrap() as u8);

    if with_spelled_letters {
        alt((
            single_digit_parse,
            map(tag("one"), |_| 1),
            map(tag("two"), |_| 2),
            map(tag("three"), |_| 3),
            map(tag("four"), |_| 4),
            map(tag("five"), |_| 5),
            map(tag("six"), |_| 6),
            map(tag("seven"), |_| 7),
            map(tag("eight"), |_| 8),
            map(tag("nine"), |_| 9),
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
