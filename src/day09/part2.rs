use itertools::Itertools;

use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let histories = parse(input);

    let extrapolated_history_values = histories
        .into_iter()
        .map(|history| extrapolate_history_value(history))
        .collect_vec();

    let value = extrapolated_history_values.iter().sum::<i32>();

    if use_sample {
        assert_eq!(value, 2);
    } else {
        println!("{:?}", value);
    }
}

fn extrapolate_history_value(history: History) -> i32 {
    let differences = history
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if differences.iter().all(|d| d == &0) {
        history.first().unwrap() - 0
    } else {
        history.first().unwrap() - extrapolate_history_value(differences)
    }
}
