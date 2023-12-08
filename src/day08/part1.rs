use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let input = parse(input);

    let mut steps = 0u32;
    let mut current_node = "AAA".to_string();

    loop {
        for index in 0..input.instructions.len() {
            let instruction = input.instructions.get(index).unwrap();

            let node: &NodeInstructions = input.nodes.get(&current_node).unwrap();

            match instruction {
                Instruction::Left => {
                    current_node = node.left.to_string();
                }
                Instruction::Right => {
                    current_node = node.right.to_string();
                }
            }

            steps += 1;

            if current_node == "ZZZ" {
                break;
            }
        }

        if current_node == "ZZZ" {
            break;
        }
    }

    let value = steps;

    if use_sample {
        assert_eq!(value, 6);
    } else {
        println!("{:?}", value);
    }
}
