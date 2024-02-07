use regex::Regex;
use std::collections::HashMap;

const END_NODE: &str = "ZZZ";

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

fn travers_the_network(navigation_instruction: &str, network: HashMap<&str, Destination>) -> u32 {
    let mut start_node = "AAA";
    let instructions = navigation_instruction.chars();
    let mut step = 0_u32;

    loop {
        let mut destination = network.get(start_node).unwrap();
        let mut next_destination = "";

        for instruction in instructions.clone() {
            next_destination = match instruction {
                'L' => destination.left.as_str(),
                'R' => destination.right.as_str(),
                _ => panic!("Uknown instruction"),
            };

            // println!("{:}: {}", step, next_destination);

            step += 1;

            if next_destination == END_NODE {
                return step;
            }

            destination = network.get(next_destination).unwrap();
        }

        start_node = next_destination;
    }
}

fn parse_input(input: &str) -> (&str, HashMap<&str, Destination>) {
    let mut network = HashMap::new();
    let lines = input.trim().split('\n');
    let mut navigation_instructions = "";
    let re = Regex::new("[A-Z]+").unwrap();

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
                left: String::from(destinations.next().unwrap()),
                right: String::from(destinations.next().unwrap()),
            },
        );
    }

    (navigation_instructions, network)
}
