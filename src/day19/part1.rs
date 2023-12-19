use super::common::*;

pub fn run(input: &str, use_sample: bool) {
    let Input { workflows, ratings } = parse(input);

    let entry_workflow = workflows.get("in").unwrap();

    let value = ratings
        .iter()
        .map(|rating| {
            let accepted = resolve_workflow(entry_workflow, &rating, &workflows);

            if accepted {
                rating.values().sum::<u32>()
            } else {
                0
            }
        })
        .sum::<u32>();

    if use_sample {
        assert_eq!(value, 19114);
    } else {
        println!("{}", value);
    }
}

fn resolve_workflow(workflow: &Workflow, rating: &Rating, workflows: &Workflows) -> bool {
    for rule in &workflow.rules {
        let rating_value = rating.get(&rule.part).unwrap();

        let accepted = match rule.operator {
            RuleOperator::GreaterThan => rating_value > &rule.value,
            RuleOperator::LessThan => rating_value < &rule.value,
        };

        if accepted {
            let resolved = match rule.result.clone() {
                WorkflowResult::Accepted => true,
                WorkflowResult::Rejected => false,
                WorkflowResult::Workflow(name) => {
                    let next_workflow = workflows.get(&name).unwrap();
                    resolve_workflow(next_workflow, rating, workflows)
                }
            };

            return resolved;
        }
    }

    match workflow.last.clone() {
        WorkflowResult::Accepted => true,
        WorkflowResult::Rejected => false,
        WorkflowResult::Workflow(name) => {
            let next_workflow = workflows.get(&name).unwrap();
            resolve_workflow(next_workflow, rating, workflows)
        }
    }
}
