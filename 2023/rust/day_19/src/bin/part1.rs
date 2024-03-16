use std::collections::HashMap;

use regex::Regex;

const STARTING_WORKFLOW: &str = "in";

#[derive(Debug)]
struct Part {
    categories: HashMap<char, u32>,
}

#[derive(Debug)]
struct OrgSystem {
    workflows: HashMap<String, String>,
}

impl OrgSystem {
    fn parse_rules(rules: &str, part: &Part) -> Option<String> {
        for rule in rules.split(',') {
            if !rule.contains(':') {
                return Some(rule.to_owned());
            }

            let (category, rest) = rule.split_at(1);
            let (operator, rest) = rest.split_at(1);
            let (value, next_step) = rest.split_once(':').unwrap();

            let category = category.chars().next().unwrap();
            let operator = operator.chars().next().unwrap();
            let value = value.parse::<u32>().unwrap();

            let category_value = part.categories.get(&category).unwrap();

            match operator {
                '>' => {
                    if *category_value > value {
                        return Some(next_step.to_owned());
                    }
                }
                '<' => {
                    if *category_value < value {
                        return Some(next_step.to_owned());
                    }
                }
                _ => panic!("Unknwon operator"),
            }
        }

        None
    }

    fn apply_part(&self, part: &Part) -> Option<u32> {
        let mut workflow_rules = self.workflows.get(STARTING_WORKFLOW).unwrap();

        loop {
            let next_state = Self::parse_rules(workflow_rules.as_str(), part).unwrap();

            if next_state == "A" {
                return Some(part.categories.iter().fold(0, |a, b| a + *b.1));
            } else if next_state == "R" {
                return None;
            }

            workflow_rules = self.workflows.get(&next_state).unwrap();
        }
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let (workflows, parts) = parse_input(input);
    let mut sum = 0;
    for part in parts {
        if let Some(rating_sum) = workflows.apply_part(&part) {
            sum += rating_sum;
        }
    }
    dbg!(sum);
}

fn parse_input(input: &str) -> (OrgSystem, Vec<Part>) {
    let mut parts_rating_section_reached = false;
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    for line in input.trim().split('\n') {
        if line.is_empty() {
            parts_rating_section_reached = true;
            continue;
        }

        if parts_rating_section_reached {
            let re = Regex::new(r"\d+").unwrap();
            let mut ratings = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<u32>().unwrap());

            let mut categories = HashMap::new();
            categories.insert('x', ratings.next().unwrap());
            categories.insert('m', ratings.next().unwrap());
            categories.insert('a', ratings.next().unwrap());
            categories.insert('s', ratings.next().unwrap());
            parts.push(Part { categories });
        } else {
            let (id, rules) = line.split_once('{').unwrap();
            workflows.insert(id.to_owned(), rules.strip_suffix('}').unwrap().to_owned());
        }
    }

    (OrgSystem { workflows }, parts)
}
