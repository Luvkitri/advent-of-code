use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct FlipFlopModule {
    is_on: bool,
}

impl FlipFlopModule {
    fn handle_pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.is_on = !self.is_on;
                if self.is_on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ConjunctionModule {
    pulses: HashMap<String, Pulse>,
}

type IdsMapping = HashMap<String, Vec<String>>;
type FlilFlopMapping = HashMap<String, FlipFlopModule>;
type ConjunctionMapping = HashMap<String, ConjunctionModule>;

impl ConjunctionModule {
    fn handle_pulse(&mut self, module_id: String, pulse: Pulse) -> Pulse {
        self.pulses.insert(module_id, pulse);

        if self.pulses.iter().any(|(_, p)| *p == Pulse::Low) {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

fn main() {
    dbg!(count_button_presses());
}

fn count_button_presses() -> usize {
    let input = include_str!("input3.txt");
    let (ids_mapping, mut flip_flop_modules, mut conjunction_modules) = parse_input(input);

    // let mut low_pulse_count = 0_u32;
    // let mut high_pulse_count = 0_u32;
    let mut count = 0;
    let mut counts = Vec::new();
    loop {
        count += 1;
        let mut modules_queue: VecDeque<(String, String, Pulse)> = VecDeque::from(
            ids_mapping
                .get("broadcaster")
                .unwrap()
                .iter()
                .map(|id| (String::from("broadcaster"), id.clone(), Pulse::Low))
                .collect::<Vec<(String, String, Pulse)>>(),
        );

        while let Some((previous_id, current_id, current_pulse)) = modules_queue.pop_front() {
            if current_pulse == Pulse::Low
                && (current_id == "ph"
                    || current_id == "vn"
                    || current_id == "kt"
                    || current_id == "hn")
            {
                println!(
                    "Iter {:}: {} -{:?}-> {}",
                    count, previous_id, current_pulse, current_id
                );
                counts.push(count);
            }

            if counts.len() == 4 {
                let mut counts = counts.iter();
                let first = *counts.next().unwrap();
                let second = *counts.next().unwrap();
                let mut current_lcm = lcm(first, second);

                for &next in counts {
                    current_lcm = lcm(current_lcm, next);
                }

                return current_lcm;
            }

            let next_pulse = if let Some(flip_flop) = flip_flop_modules.get_mut(&current_id) {
                flip_flop.handle_pulse(current_pulse)
            } else {
                conjunction_modules
                    .get_mut(&current_id)
                    .map(|confjunction| confjunction.handle_pulse(previous_id, current_pulse))
            };

            if let Some((next_ids, pulse)) = ids_mapping
                .get(&current_id)
                .and_then(|next_ids| next_pulse.map(|p| (next_ids.clone(), p)))
            {
                modules_queue.extend(
                    next_ids
                        .iter()
                        .map(|id| (current_id.clone(), id.clone(), pulse)),
                );
            }
        }
    }
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        return x;
    }

    gcd(y, x % y)
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn parse_input(input: &str) -> (IdsMapping, FlilFlopMapping, ConjunctionMapping) {
    let mut ids_mapping = HashMap::new();
    let mut flip_flop_modules = HashMap::new();
    let mut conjunction_modules = HashMap::new();
    for line in input.trim().split('\n') {
        let (id, ids) = line.split_once(" -> ").unwrap();
        let mut id = id.to_owned();

        if id.contains('%') {
            id.remove(0);
            flip_flop_modules.insert(id.clone(), FlipFlopModule { is_on: false });
        } else if id.contains('&') {
            id.remove(0);
            conjunction_modules.insert(
                id.clone(),
                ConjunctionModule {
                    pulses: HashMap::new(),
                },
            );
        }

        ids_mapping.insert(
            id,
            ids.split(", ")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        );
    }

    for (outcoming_id, ids) in ids_mapping.clone() {
        for id in ids {
            conjunction_modules.get_mut(&id).map(|module| {
                module
                    .pulses
                    .entry(outcoming_id.clone())
                    .or_insert(Pulse::Low)
            });
        }
    }

    (ids_mapping, flip_flop_modules, conjunction_modules)
}
