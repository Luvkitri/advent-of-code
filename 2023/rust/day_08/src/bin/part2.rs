use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Destination {
    left: String,
    right: String,
}

fn main() {
    let input = include_str!("input3.txt");
    let (navigation_instructions, network) = parse_input(input);
    let step_count = travers_the_network(navigation_instructions, network);
    dbg!(step_count);
}

fn travers_the_network(navigation_instruction: &str, network: HashMap<&str, Destination>) -> usize {
    let start_nodes = get_all_start_nodes(&network);
    let mut counts = Vec::new();

    for start_node in start_nodes {
        let mut current_node = start_node.clone();
        let mut count = 0;

        let mut instructions = navigation_instruction.chars();

        while !current_node.ends_with('Z') {
            let mut next_instruction = instructions.next();

            if next_instruction.is_none() {
                instructions = navigation_instruction.chars();
                next_instruction = instructions.next();
            }

            if next_instruction.is_some_and(|i| i == 'L') {
                current_node = network.get(current_node.as_str()).unwrap().left.clone();
            } else if next_instruction.is_some_and(|i| i == 'R') {
                current_node = network.get(current_node.as_str()).unwrap().right.clone();
            }

            count += 1;
        }

        println!("{} -> {} in {:}", start_node, current_node, count);
        counts.push(count);
    }

    let mut counts = counts.iter();
    let first = *counts.next().unwrap();
    let second = *counts.next().unwrap();
    let mut current_lcm = lcm(first, second);

    for &next in counts {
        current_lcm = lcm(current_lcm, next);
    }

    current_lcm
}

fn parse_input(input: &str) -> (&str, HashMap<&str, Destination>) {
    let mut network = HashMap::new();
    let lines = input.trim().split('\n');
    let mut navigation_instructions = "";
    let re = Regex::new("[A-Z0-9]+").unwrap();

    for (index, line) in lines.enumerate() {
        if index == 0 {
            navigation_instructions = line;
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let (location, destinations) = line.split_once('=').unwrap();
        let mut destinations = re.find_iter(destinations).map(|m| m.as_str());
        network.insert(
            location.trim(),
            Destination {
                left: destinations.next().unwrap().to_string(),
                right: destinations.next().unwrap().to_string(),
            },
        );
    }

    (navigation_instructions, network)
}

fn get_all_start_nodes(network: &HashMap<&str, Destination>) -> Vec<String> {
    let mut start_nodes = Vec::new();
    for key in network.keys() {
        if key.ends_with('A') {
            start_nodes.push(key.to_string());
        }
    }
    start_nodes
}

// Greatest common divisor
fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        return x;
    }

    gcd(y, x % y)
}

// Lowest commmon multiple
fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}
