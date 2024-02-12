#[derive(Clone, Copy)]
struct Galaxy {
    i: i64,
    j: i64,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u64 {
        self.i.abs_diff(other.i) + self.j.abs_diff(other.j)
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let galaxies = parse_input(input);
    let mut total_distance = 0;

    for (index, galaxy) in galaxies.clone().into_iter().enumerate() {
        let start = index + 1;

        if start == galaxies.len() {
            break;
        }

        for i in start..galaxies.len() {
            total_distance += galaxy.distance(galaxies.get(i).unwrap());
        }
    }

    dbg!(total_distance);
}

fn parse_input(input: &str) -> Vec<Galaxy> {
    let mut empty_rows_positions = Vec::new();
    for (i, line) in input.trim().split('\n').enumerate() {
        let row = line.chars().collect::<Vec<char>>();

        if !row.contains(&'#') {
            empty_rows_positions.push(i);
        }
    }

    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut empty_columns_positions = Vec::new();
    for j in 0..lines.first().unwrap().len() {
        let mut column = Vec::new();
        for line in lines.clone() {
            column.push(line.get(j..j + 1).unwrap().chars().next().unwrap());
        }

        if !column.contains(&'#') {
            empty_columns_positions.push(j);
        }
    }

    let mut galaxies = Vec::new();
    let mut i_offset = 0_i64;
    for (i, row) in input.trim().split('\n').enumerate() {
        let mut j_offset = 0_i64;
        if empty_rows_positions.contains(&i) {
            i_offset += 999_999;
        }

        for (j, character) in row.chars().enumerate() {
            if empty_columns_positions.contains(&j) {
                j_offset += 999_999;
            }

            if character == '#' {
                galaxies.push(Galaxy {
                    i: (i as i64) + i_offset,
                    j: (j as i64) + j_offset,
                });
            }
        }
    }

    galaxies
}
