struct HotSpringRecord {
    springs_mapping: String,
    groups: String,
}

impl HotSpringRecord {
    fn get_possible_springs_mapping(&self) -> Vec<String> {
        let mut mappings = Vec::new();

        let question_mark_character_count: usize = self
            .springs_mapping
            .chars()
            .map(|c| if c == '?' { 1 } else { 0 })
            .sum();

        dbg!(question_mark_character_count);

        for i in 0..question_mark_character_count + 1 {
            for j in 0..question_mark_character_count + 1 {
                let mapping = self
                    .springs_mapping
                    .replacen('?', ".", i)
                    .replacen('?', "#", j)
                    .replace('?', ".");

                dbg!(mapping.clone());
                if self.is_matching(mapping.as_str()) {
                    mappings.push(mapping);
                }
            }
        }

        mappings
    }

    fn is_matching(&self, other_mapping: &str) -> bool {
        let mut other_groups = String::from("");
        let mut count = 0;
        for character in other_mapping.chars() {
            if character == '#' {
                count += 1;
            }

            if count > 0 && character == '.' {
                other_groups += (count.to_string() + ",").as_str();
                count = 0;
            }
        }
        if count > 0 {
            other_groups += (count.to_string() + ",").as_str();
        }

        other_groups.remove(other_groups.len() - 1);
        dbg!(other_groups.clone());

        other_groups == self.groups
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let records = parse_input(input);
    dbg!(records.first().unwrap().get_possible_springs_mapping());
}

fn parse_input(input: &str) -> Vec<HotSpringRecord> {
    let mut records = Vec::new();

    for line in input.trim().split('\n') {
        let (springs_mapping, groups) = line.split_once(' ').unwrap();

        records.push(HotSpringRecord {
            springs_mapping: springs_mapping.to_string(),
            groups: groups.to_string(),
        });
    }

    records
}
