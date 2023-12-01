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
                    Ok((_, digit)) => {
                        first_digit = digit;
                        break;
                    }
                    Err(_) => continue,
                }
            }

            let mut last_digit: u8 = 0;

            for index in (0..line_len).rev() {
                let text = &line[index..];

                match parse_digit(text, with_spelled_letters) {
                    Ok((_, digit)) => {
                        last_digit = digit;
                        break;
                    }
                    Err(_) => continue,
                }
            }

            CalibrationValue {
                first_digit,
                last_digit,
            }
        })
        .collect()
}

fn parse_digit(input: &str, with_spelled_letters: bool) -> IResult<&str, ParsedDigit> {
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
