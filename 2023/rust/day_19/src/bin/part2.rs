use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Part {
    categories: HashMap<char, (u64, u64)>,
}

impl Part {
    fn generate_new_part(&self, key: char, threshold: u64, operator: char) -> (Part, Part) {
        let mut above_threshold = self.categories.clone();
        let mut below_threshold = self.categories.clone();

        match operator {
            '>' => {
                above_threshold
                    .entry(key)
                    .and_modify(|p| *p = (threshold + 1, p.1));
                below_threshold
                    .entry(key)
                    .and_modify(|p| *p = (p.0, threshold));
            }
            '<' => {
                above_threshold
                    .entry(key)
                    .and_modify(|p| *p = (threshold, p.1));
                below_threshold
                    .entry(key)
                    .and_modify(|p| *p = (p.0, threshold - 1));
            }
            _ => panic!("Unknown operator"),
        }

        (
            Part {
                categories: above_threshold,
            },
            Part {
                categories: below_threshold,
            },
        )
    }

    fn get_combinations(&self) -> u64 {
        self.categories
            .clone()
            .into_values()
            .filter(|range| range.0 < range.1)
            .map(|range| range.1 - range.0 + 1)
            .reduce(|acc, e| acc * e)
            .unwrap()
    }
}

#[derive(Debug)]
struct OrgSystem {
    workflows: HashMap<String, String>,
}

impl OrgSystem {
    fn parse_rules(rules: &str, part: Part) -> Vec<(String, Part)> {
        let mut steps: Vec<(String, Part)> = Vec::new();
        let mut next_part = part;
        for rule in rules.split(',') {
            if !rule.contains(':') {
                steps.push((rule.to_owned(), next_part));
                break;
            }

            let (category, rest) = rule.split_at(1);
            let (operator, rest) = rest.split_at(1);
            let (value, next_step) = rest.split_once(':').unwrap();

            let category = category.chars().next().unwrap();
            let operator = operator.chars().next().unwrap();
            let value = value.parse::<u64>().unwrap();

            let (above_part, below_part) = next_part.generate_new_part(category, value, operator);

            match operator {
                '>' => {
                    steps.push((next_step.to_owned(), above_part));
                    next_part = below_part;
                }
                '<' => {
                    steps.push((next_step.to_owned(), below_part));
                    next_part = above_part;
                }
                _ => panic!("Unknwon operator"),
            }
        }

        steps
    }

    fn apply_part(&self, part: &Part) -> u64 {
        let mut steps = vec![(String::from("in"), part.clone())];
        let mut sum = 0_u64;
        while let Some((step, current_part)) = steps.pop() {
            if step == "A" {
                sum += current_part.get_combinations();
                continue;
            } else if step == "R" {
                continue;
            }
            let workflow_rules = self.workflows.get(&step).unwrap();
            steps.append(&mut Self::parse_rules(
                workflow_rules.as_str(),
                current_part,
            ));
        }
        sum
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let workflows = parse_input(input);
    let initial_part = Part {
        categories: HashMap::from([
            ('x', (1, 4000)),
            ('m', (1, 4000)),
            ('a', (1, 4000)),
            ('s', (1, 4000)),
        ]),
    };
    let sum = workflows.apply_part(&initial_part);
    dbg!(sum);
}

fn parse_input(input: &str) -> OrgSystem {
    let mut workflows = HashMap::new();
    for line in input.trim().split('\n') {
        if line.is_empty() {
            break;
        }

        let (id, rules) = line.split_once('{').unwrap();
        workflows.insert(id.to_owned(), rules.strip_suffix('}').unwrap().to_owned());
    }
    OrgSystem { workflows }
}
