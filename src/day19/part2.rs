use itertools::Itertools;

use super::common::*;

const MIN_VALUE: u32 = 1;
const MAX_VALUE: u32 = 4000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CombinationRule {
    part: Part,
    operator: RuleOperator,
    value: u32,
}

type AcceptedCombination = Vec<CombinationRule>;
type AcceptedCombinations = Vec<AcceptedCombination>;

pub fn run(input: &str, use_sample: bool) {
    let Input { workflows, .. } = parse(input);

    let entry_workflow = workflows.get("in").unwrap();

    let accepted_combinations = get_combinations(entry_workflow, &workflows, vec![]);

    let total_ratings = accepted_combinations
        .iter()
        .map(|combination| {
            let (min_x, max_x) = get_combination_range(combination, Part::X);
            let (min_m, max_m) = get_combination_range(combination, Part::M);
            let (min_a, max_a) = get_combination_range(combination, Part::A);
            let (min_s, max_s) = get_combination_range(combination, Part::S);

            (max_x - min_x + 1) as u64
                * (max_m - min_m + 1) as u64
                * (max_a - min_a + 1) as u64
                * (max_s - min_s + 1) as u64
        })
        .sum::<u64>();

    let value = total_ratings;

    if use_sample {
        assert_eq!(value, 167409079868000);
    } else {
        println!("{}", value);
    }
}

fn get_combination_range(combination: &Vec<CombinationRule>, part: Part) -> (u32, u32) {
    let mut min = MIN_VALUE;
    let mut max = MAX_VALUE;

    for rule in combination.iter().filter(|c| c.part == part) {
        match rule.operator {
            RuleOperator::GreaterThan => {
                if rule.value > min {
                    min = rule.value + 1;
                }
            }
            RuleOperator::LessThan => {
                if rule.value < max {
                    max = rule.value - 1;
                }
            }
        }
    }

    (min, max)
}

fn get_combinations(
    current_workflow: &Workflow,
    workflows: &Workflows,
    rules: Vec<CombinationRule>,
) -> AcceptedCombinations {
    let head = &current_workflow.rules[0];
    let tail = &current_workflow.rules[1..];

    let head_rules = rules
        .clone()
        .into_iter()
        .chain(vec![CombinationRule {
            part: head.part.clone(),
            operator: head.operator.clone(),
            value: head.value,
        }])
        .collect_vec();

    let rules_from_head = match head.result.clone() {
        WorkflowResult::Accepted => vec![head_rules],
        WorkflowResult::Rejected => vec![],
        WorkflowResult::Workflow(name) => {
            let next_workflow = workflows.get(&name).unwrap();
            get_combinations(next_workflow, workflows, head_rules)
        }
    };

    let opposite_rule = get_opposite_rule(head);

    if tail.is_empty() {
        let last_rules = rules
            .clone()
            .into_iter()
            .chain(vec![opposite_rule])
            .collect_vec();

        let rules_from_last = match current_workflow.last.clone() {
            WorkflowResult::Accepted => vec![last_rules],
            WorkflowResult::Rejected => vec![],
            WorkflowResult::Workflow(name) => {
                let next_workflow = workflows.get(&name).unwrap();
                get_combinations(next_workflow, workflows, last_rules)
            }
        };

        rules_from_head
            .into_iter()
            .chain(rules_from_last)
            .collect_vec()
    } else {
        let tail_rules = rules
            .clone()
            .into_iter()
            .chain(vec![opposite_rule])
            .collect_vec();

        let fake_workflow = Workflow {
            name: current_workflow.name.clone(),
            rules: tail.to_vec(),
            last: current_workflow.last.clone(),
        };
        let tail_combinations = get_combinations(&fake_workflow, workflows, tail_rules);

        rules_from_head
            .into_iter()
            .chain(tail_combinations)
            .collect_vec()
    }
}

fn get_opposite_rule(head: &Rule) -> CombinationRule {
    let last_rule_operator = match head.operator {
        RuleOperator::GreaterThan => RuleOperator::LessThan,
        RuleOperator::LessThan => RuleOperator::GreaterThan,
    };
    let last_rule_value = match head.operator {
        RuleOperator::GreaterThan => head.value + 1,
        RuleOperator::LessThan => head.value - 1,
    };

    CombinationRule {
        part: head.part.clone(),
        operator: last_rule_operator,
        value: last_rule_value,
    }
}
