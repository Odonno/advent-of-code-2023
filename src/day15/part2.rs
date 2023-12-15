use itertools::Itertools;
use std::collections::HashMap;

use super::common::*;

#[derive(Debug, Clone, PartialEq)]
struct Lens {
    label: String,
    focal_length: u32,
}

type BoxNumber = u8;
type Boxes = HashMap<BoxNumber, Vec<Lens>>;

pub fn run(input: &str, use_sample: bool) {
    let sequences = parse(input);

    let mut boxes = Boxes::new();

    for sequence in sequences {
        if sequence.contains("=") {
            let pairs = sequence.split('=').collect_vec();

            let name = pairs[0].to_string();
            let focal_length = pairs[1].parse::<u32>().unwrap();
            let box_index = calculate_box_index(&name);

            let lens = Lens {
                label: name.to_string(),
                focal_length,
            };

            boxes
                .entry(box_index as u8)
                .and_modify(|_box| {
                    let is_in_box = _box.iter().any(|lens| lens.label == name);

                    if is_in_box {
                        _box.iter_mut()
                            .find(|lens| lens.label == name)
                            .unwrap()
                            .focal_length = focal_length;
                    } else {
                        _box.push(lens.clone());
                    }
                })
                .or_insert(vec![lens]);
        }

        if sequence.contains("-") {
            let pairs = sequence.split('-').collect_vec();

            let name = pairs[0].to_string();
            let box_index = calculate_box_index(&name);

            boxes.entry(box_index as u8).and_modify(|_box| {
                _box.retain(|lens| lens.label != name);
            });
        }
    }

    let value = boxes
        .iter()
        .map(|(box_index, lenses)| {
            let box_value = box_index.clone() as u32 + 1;

            lenses
                .iter()
                .enumerate()
                .map(|(lens_index, lens)| box_value * (lens_index as u32 + 1) * lens.focal_length)
                .sum::<u32>()
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 145);
    } else {
        println!("{}", value);
    }
}

fn calculate_box_index(name: &String) -> u8 {
    let mut box_index = 0;

    for char in name.chars() {
        box_index += char as u16;
        box_index *= 17;
        box_index %= 256;
    }

    box_index as u8
}
