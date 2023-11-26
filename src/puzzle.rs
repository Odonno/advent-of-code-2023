use chrono::Utc;
use colored::Colorize;

pub fn run(day: Option<u8>, part: Option<u8>, use_sample: Option<bool>) {
    let before = Utc::now();

    let env_day = env!("DAY").parse::<u8>().unwrap();
    let env_part = env!("PART").parse::<u8>().unwrap();
    let env_use_sample = env!("USE_SAMPLE").parse::<bool>().unwrap();

    let day = day.unwrap_or(env_day);
    let part = part.unwrap_or(env_part);
    let use_sample = use_sample.unwrap_or(env_use_sample);

    let run_fn = match day {
        1 => day01,
        2 => day02,
        3 => day03,
        4 => day04,
        5 => day05,
        6 => day06,
        7 => day07,
        8 => day08,
        9 => day09,
        10 => day10,
        11 => day11,
        12 => day12,
        13 => day13,
        14 => day14,
        15 => day15,
        16 => day16,
        17 => day17,
        18 => day18,
        19 => day19,
        20 => day20,
        21 => day21,
        22 => day22,
        23 => day23,
        24 => day24,
        25 => day25,
        _ => panic!("Invalid day number. Did you forget to generate this day using the script?"),
    };

    run_fn(part, use_sample);

    let after = Utc::now();

    let duration = after - before;
    let duration_text = match duration.num_seconds() {
        seconds if seconds > 3 => format!("Took {} seconds. Really slow...", seconds).italic(),
        seconds if seconds > 0 => format!("Took {}s. A bit slow, right?", seconds).italic(),
        _ => format!("Took {}ms", duration.num_milliseconds()).italic(),
    };

    println!("{}", duration_text);
}

use crate::day01::run as day01;
use crate::day02::run as day02;
use crate::day03::run as day03;
use crate::day04::run as day04;
use crate::day05::run as day05;
use crate::day06::run as day06;
use crate::day07::run as day07;
use crate::day08::run as day08;
use crate::day09::run as day09;
use crate::day10::run as day10;
use crate::day11::run as day11;
use crate::day12::run as day12;
use crate::day13::run as day13;
use crate::day14::run as day14;
use crate::day15::run as day15;
use crate::day16::run as day16;
use crate::day17::run as day17;
use crate::day18::run as day18;
use crate::day19::run as day19;
use crate::day20::run as day20;
use crate::day21::run as day21;
use crate::day22::run as day22;
use crate::day23::run as day23;
use crate::day24::run as day24;
use crate::day25::run as day25;
