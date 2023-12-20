use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(20);

#[derive(Debug, Clone)]
struct Module {
    variant: ModuleType,
    targets: Vec<String>,
}

impl Module {
    fn state_hash(&self) -> String {
        match self.variant {
            ModuleType::FlipFlop { state } => format!("{}", state),
            ModuleType::Conjuction { ref input_states } => input_states
                .iter()
                .sorted_by_key(|(name, _)| *name)
                .map(|(_, state)| format!("{:?}", state))
                .join(""),
            _ => String::from(""),
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop {
        state: bool,
    },
    Conjuction {
        input_states: HashMap<String, Pulse>,
    },
    Untyped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    for line in input.lines() {
        let (module, targets) = line.split(" -> ").collect_tuple().unwrap();
        let (name, variant) = if module == "broadcaster" {
            (module, ModuleType::Broadcaster)
        } else if module.starts_with('%') {
            (&module[1..], ModuleType::FlipFlop { state: false })
        } else if module.starts_with('&') {
            (
                &module[1..],
                ModuleType::Conjuction {
                    input_states: HashMap::new(),
                },
            )
        } else {
            (module, ModuleType::Untyped)
        };

        let targets = targets.split(", ").map(String::from).collect_vec();

        let module = Module { variant, targets };
        modules.insert(name.to_string(), module);
    }

    for (name, module) in modules.clone() {
        for target in module.targets {
            match modules.get_mut(&target) {
                Some(Module {
                    variant: ModuleType::Conjuction { input_states },
                    ..
                }) => {
                    input_states.insert(name.to_string(), Pulse::Low);
                }
                _ => {}
            }
        }
    }

    modules
}

fn press_button(mut modules: HashMap<String, Module>) -> (HashMap<String, Module>, (u64, u64)) {
    let mut queue = VecDeque::new();
    queue.push_back((
        String::from("broadcaster"),
        Pulse::Low,
        String::from("button"),
    ));
    let mut pulses = (0, 0);

    while let Some((name, pulse, from)) = queue.pop_front() {
        process_pulse(pulse, &mut pulses, &mut modules, name, &mut queue, from);
    }

    (modules, pulses)
}

fn press_button_2(
    mut modules: HashMap<String, Module>,
    mut check: impl FnMut(String, Pulse, String),
) -> HashMap<String, Module> {
    let mut queue = VecDeque::new();
    queue.push_back((
        String::from("broadcaster"),
        Pulse::Low,
        String::from("button"),
    ));
    let mut pulses = (0, 0);

    while let Some((name, pulse, from)) = queue.pop_front() {
        process_pulse(
            pulse,
            &mut pulses,
            &mut modules,
            name.clone(),
            &mut queue,
            from.clone(),
        );
        check(name, pulse, from);
    }

    modules
}

fn process_pulse(
    pulse: Pulse,
    pulses: &mut (u64, u64),
    modules: &mut HashMap<String, Module>,
    name: String,
    queue: &mut VecDeque<(String, Pulse, String)>,
    from: String,
) {
    match pulse {
        Pulse::Low => pulses.0 += 1,
        Pulse::High => pulses.1 += 1,
    }
    let Some(module) = modules.get_mut(&name) else {
        return;
    };
    match module.variant {
        ModuleType::Broadcaster => {
            for target in module.targets.iter() {
                queue.push_back((target.clone(), pulse, name.clone()));
            }
        }
        ModuleType::FlipFlop { ref mut state } => {
            if pulse == Pulse::High {
                return;
            }
            *state = !*state;
            for target in module.targets.iter() {
                queue.push_back((
                    target.clone(),
                    if *state { Pulse::High } else { Pulse::Low },
                    name.clone(),
                ));
            }
        }
        ModuleType::Conjuction {
            ref mut input_states,
        } => {
            input_states.insert(from.clone(), pulse);
            let out_pulse = if input_states.values().all(|&state| state == Pulse::High) {
                Pulse::Low
            } else {
                Pulse::High
            };
            for target in module.targets.iter() {
                queue.push_back((target.clone(), out_pulse, name.clone()));
            }
        }
        ModuleType::Untyped => {
            println!("{name}: {pulse:?}");
        }
    }
}

fn hash_modules_states(modules: &HashMap<String, Module>) -> String {
    modules
        .iter()
        .sorted_by_key(|(name, _)| *name)
        .map(|(_, module)| module.state_hash())
        .join(";")
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut modules = parse_modules(input);

    let mut prev_states = Vec::new();
    let mut states_pulses = HashMap::new();
    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;

    let hash = hash_modules_states(&modules);
    prev_states.push(hash);

    for i in 0..1000 {
        let (m, pulses) = press_button(modules);
        modules = m;
        total_low_pulses += pulses.0;
        total_high_pulses += pulses.1;

        let hash = hash_modules_states(&modules);
        states_pulses.insert(hash.clone(), pulses);
        if prev_states.contains(&hash) {
            let loop_start = prev_states.iter().position(|s| s == &hash).unwrap();
            let loop_end = prev_states.len();
            let loop_length = loop_end - loop_start;
            (total_low_pulses, total_high_pulses) = (i + 1..1000)
                .map(|i| i % loop_length)
                .map(|i| states_pulses[&prev_states[i]])
                .fold(
                    (total_low_pulses, total_high_pulses),
                    |(low, high), (l, h)| (low + l, high + h),
                );

            break;
        }
        prev_states.push(hash);
    }

    let result = total_low_pulses * total_high_pulses;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut modules = parse_modules(input);

    let pre_rx = modules
        .iter()
        .find(|(_, module)| module.targets.contains(&String::from("rx")))
        .unwrap()
        .0
        .to_string();

    let mut button_presses = 0;

    let mut unique_states = HashMap::new();

    loop {
        button_presses += 1;
        let m = press_button_2(modules, |name, pulse, from| {
            if name == pre_rx && pulse == Pulse::High {
                if !unique_states.contains_key(&name) {
                    unique_states.insert(from, button_presses);
                }
            }
        });

        match &m[&pre_rx].variant {
            ModuleType::Conjuction { input_states } => {
                if unique_states.len() == input_states.len() {
                    break;
                }
            }
            _ => {}
        }

        modules = m;
    }

    let lcm = unique_states
        .values()
        .fold(1, |acc, &x| num::integer::lcm(acc, x));
    Some(lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(32000000));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(238593356738827));
    }
}
