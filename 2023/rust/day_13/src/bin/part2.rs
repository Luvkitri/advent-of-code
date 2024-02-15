use std::cmp;

#[derive(Clone, Debug)]
struct Terrain {
    mapping: Vec<String>,
}

impl Terrain {
    fn get_reflection_count(&self, is_row: bool) -> Option<usize> {
        let mut offset = 0_usize;
        loop {
            let previous_rows_breakpoint = self.get_previous_rows_breakpoint(offset);

            previous_rows_breakpoint?;

            let (previous_rows, next_rows) =
                self.mapping.split_at(previous_rows_breakpoint.unwrap());
            let mut previous_rows = previous_rows.to_owned();
            previous_rows.reverse();
            let mut next_rows = next_rows.to_owned();

            let smudge_found = Self::fix_smudge(&mut previous_rows, &mut next_rows);
            dbg!(previous_rows.clone(), next_rows.clone());
            dbg!(smudge_found);

            if smudge_found && Self::is_reflection(previous_rows.as_slice(), next_rows.as_slice()) {
                return if is_row {
                    Some(previous_rows.len() * 100)
                } else {
                    Some(previous_rows.len())
                };
            }

            offset = previous_rows.len();
        }
    }

    fn is_reflection(first_part: &[String], second_part: &[String]) -> bool {
        let shorter_len = cmp::min(first_part.len(), second_part.len());

        for i in 0..shorter_len {
            if first_part.get(i).unwrap() != second_part.get(i).unwrap() {
                return false;
            }
        }

        true
    }

    fn get_previous_rows_breakpoint(&self, offset: usize) -> Option<usize> {
        let mut previous_rows: Vec<String> = Vec::new();
        for i in offset..self.mapping.len() {
            let row = self.mapping.get(i).unwrap().clone();

            if previous_rows
                .last()
                .is_some_and(|p| *p == row || Self::count_differences(&row, p) == 1)
            {
                return Some(previous_rows.len() + offset);
            }

            previous_rows.push(row);
        }

        None
    }

    fn fix_smudge(first_part: &mut [String], second_part: &mut [String]) -> bool {
        let shorter_len = cmp::min(first_part.len(), second_part.len());

        for i in 0..shorter_len {
            let first_part_row = first_part.get(i).unwrap();
            let second_part_row = second_part.get(i).unwrap();
            if first_part_row != second_part_row {
                let fixed_row = Self::compare_rows(first_part_row, second_part_row);
                if fixed_row.is_none() {
                    return false;
                }

                first_part[i] = fixed_row.clone().unwrap();
                second_part[i] = fixed_row.unwrap();
                return true;
            }
        }

        false
    }

    fn compare_rows(first_part_row: &str, second_part_row: &str) -> Option<String> {
        let first_count = Self::count_hashes(first_part_row);
        let second_count = Self::count_hashes(second_part_row);

        let difference_count = Self::count_differences(first_part_row, second_part_row);

        if difference_count != 1 {
            return None;
        }

        if first_count > second_count {
            return Some(second_part_row.to_string());
        }

        Some(first_part_row.to_string())
    }

    fn count_hashes(row: &str) -> u32 {
        row.chars().fold(0, |acc, e| {
            if e == '#' {
                return acc + 1;
            }

            acc
        })
    }

    fn count_differences(first_part_row: &str, second_part_row: &str) -> u32 {
        let mut count = 0_u32;
        for (first, second) in first_part_row.chars().zip(second_part_row.chars()) {
            if first != second {
                count += 1;
            }
        }
        count
    }

    fn convert_to_column_mapping(&mut self) {
        let mut columns = Vec::new();

        for i in 0..self.mapping.get(0).unwrap().len() {
            let mut column = String::from("");
            for row in self.mapping.clone() {
                column += row.get(i..i + 1).unwrap();
            }
            columns.push(column);
        }

        self.mapping = columns;
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let terrains = parse_input(input);
    let mut sum = 0;
    for mut terrain in terrains {
        let mut count = terrain.get_reflection_count(true);

        if count.is_none() {
            terrain.convert_to_column_mapping();
            count = terrain.get_reflection_count(false);
        }

        if count.is_none() {
            continue;
        }

        sum += count.unwrap();
    }

    dbg!(sum);
}

fn parse_input(input: &str) -> Vec<Terrain> {
    let mut terrains = Vec::new();
    let mut temp_mapping = Vec::new();
    for line in input.split('\n') {
        if line.is_empty() {
            terrains.push(Terrain {
                mapping: temp_mapping.clone(),
            });
            temp_mapping.clear();
            continue;
        }

        temp_mapping.push(line.to_string());
    }

    terrains
}
