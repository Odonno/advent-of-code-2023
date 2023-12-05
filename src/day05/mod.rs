mod common;
pub mod part1;
pub mod part2;

use crate::common::display_info;

const DAY: u8 = 5;

pub fn run(part: u8, use_sample: bool) {
    let input = if use_sample {
        include_str!("sample.txt")
    } else {
        include_str!("input.txt")
    };

    match part {
        1 => {
            display_info(DAY, part, use_sample);
            part1::run(input, use_sample);
        }
        2 => {
            display_info(DAY, part, use_sample);
            part2::run(input, use_sample);
        }
        _ => panic!("Invalid part number"),
    }
}
