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
            if Self::is_reflection(previous_rows.as_slice(), next_rows) {
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
        let mut previous_rows = Vec::new();
        for i in offset..self.mapping.len() {
            let row = self.mapping.get(i).unwrap().clone();

            if previous_rows.last().is_some_and(|p| *p == row) {
                return Some(previous_rows.len() + offset);
            }

            previous_rows.push(row);
        }

        None
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
