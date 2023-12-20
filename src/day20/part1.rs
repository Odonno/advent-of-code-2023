use itertools::Itertools;
use std::collections::HashMap;

use super::common::*;

type ModuleConfigurations = HashMap<String, ModuleConfiguration>;
type ModuleOrigins = HashMap<String, Vec<String>>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum FlipFlipState {
    On,
    Off,
}

type FlipFlipStates = HashMap<String, FlipFlipState>;

const TIMES: u32 = 1_000;

pub fn run(input: &str, use_sample: bool) {
    let sequence = parse(input);

    let button_configuration = ModuleConfiguration {
        module: Module {
            name: "button".to_string(),
            _type: ModuleType::Button,
        },
        destinations: vec!["broadcaster".to_string()],
    };

    let module_configurations = sequence
        .iter()
        .chain(vec![button_configuration].iter())
        .map(|c| (c.module.name.clone(), c.clone()))
        .collect::<ModuleConfigurations>();

    let module_origins = module_configurations
        .keys()
        .map(|module_name| {
            let origins = sequence
                .iter()
                .filter(|c| c.destinations.contains(module_name))
                .map(|c| c.module.name.clone())
                .collect_vec();

            (module_name.clone(), origins)
        })
        .collect::<ModuleOrigins>();

    let mut flip_flop_states = sequence
        .iter()
        .filter(|c| c.module._type == ModuleType::FlipFlop)
        .map(|c| (c.module.name.clone(), FlipFlipState::Off))
        .collect::<FlipFlipStates>();

    let mut recent_pulse_map = HashMap::new();

    let mut low_pulse_sent = 0;
    let mut high_pulse_sent = 0;

    for _ in 0..TIMES {
        let mut next_destinations = Vec::new();
        next_destinations.push(("button".to_string(), Pulse::Low));

        while !next_destinations.is_empty() {
            next_destinations = next_destinations
                .iter()
                .flat_map(|(destination, pulse)| {
                    let module_configuration = module_configurations.get(destination);
                    if module_configuration.is_none() {
                        return vec![];
                    }

                    let module_configuration = module_configuration.unwrap();

                    match module_configuration.module._type {
                        ModuleType::Button | ModuleType::Broadcaster => {
                            let next_pulse = pulse;

                            let destinations_count = module_configuration.destinations.len() as u32;

                            match next_pulse {
                                Pulse::Low => {
                                    low_pulse_sent += destinations_count;
                                }
                                Pulse::High => {
                                    high_pulse_sent += destinations_count;
                                }
                            };

                            recent_pulse_map.insert(destination.clone(), next_pulse.clone());

                            module_configuration
                                .destinations
                                .iter()
                                .map(|d| (d.clone(), next_pulse.clone()))
                                .collect_vec()
                        }
                        ModuleType::FlipFlop if pulse == &Pulse::Low => {
                            let current_state = flip_flop_states.get(destination).unwrap();

                            let next_pulse = match current_state {
                                FlipFlipState::On => Pulse::Low,
                                FlipFlipState::Off => Pulse::High,
                            };

                            let next_state = match current_state {
                                FlipFlipState::On => FlipFlipState::Off,
                                FlipFlipState::Off => FlipFlipState::On,
                            };

                            flip_flop_states.insert(destination.clone(), next_state.clone());
                            recent_pulse_map.insert(destination.clone(), next_pulse.clone());

                            let destinations_count = module_configuration.destinations.len() as u32;

                            match next_pulse {
                                Pulse::Low => {
                                    low_pulse_sent += destinations_count;
                                }
                                Pulse::High => {
                                    high_pulse_sent += destinations_count;
                                }
                            };

                            module_configuration
                                .destinations
                                .iter()
                                .map(|d| (d.clone(), next_pulse.clone()))
                                .collect_vec()
                        }
                        ModuleType::Conjunction => {
                            let conj_origins = module_origins.get(destination).unwrap();

                            let are_all_high_pulses = conj_origins.iter().all(|d| {
                                let origin_pulse = recent_pulse_map.get(d).unwrap_or(&Pulse::Low);
                                origin_pulse == &Pulse::High
                            });

                            let next_pulse = match are_all_high_pulses {
                                true => Pulse::Low,
                                false => Pulse::High,
                            };

                            let destinations_count = module_configuration.destinations.len() as u32;

                            match next_pulse {
                                Pulse::Low => {
                                    low_pulse_sent += destinations_count;
                                }
                                Pulse::High => {
                                    high_pulse_sent += destinations_count;
                                }
                            };

                            recent_pulse_map.insert(destination.clone(), next_pulse.clone());

                            module_configuration
                                .destinations
                                .iter()
                                .map(|d| (d.clone(), next_pulse.clone()))
                                .collect_vec()
                        }
                        _ => vec![],
                    }
                })
                .collect_vec();
        }
    }

    let value = low_pulse_sent * high_pulse_sent;

    if use_sample {
        assert_eq!(value, 11687500);
    } else {
        println!("{}", value);
    }
}
