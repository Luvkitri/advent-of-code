#[derive(Clone, Copy)]
struct Galaxy {
    i: i32,
    j: i32,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> u32 {
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
    let mut rows = Vec::new();
    for line in input.trim().split('\n') {
        let row = line.chars().collect::<Vec<char>>();

        if !row.contains(&'#') {
            rows.push(row.clone());
        }

        rows.push(row);
    }

    let mut temp_universe = String::from("");

    for universe_row in rows {
        temp_universe += String::from_iter(universe_row).as_str();
        temp_universe += "\n";
    }

    let lines = temp_universe.trim().split('\n').collect::<Vec<&str>>();
    let mut columns = Vec::new();
    for i in 0..lines.first().unwrap().len() {
        let mut column = Vec::new();
        for line in lines.clone() {
            column.push(line.get(i..i + 1).unwrap().chars().next().unwrap());
        }

        if !column.contains(&'#') {
            columns.push(column.clone())
        }

        columns.push(column);
    }

    let mut rows = Vec::new();

    for i in 0..columns.get(0).unwrap().len() {
        let mut row = Vec::new();
        for column in columns.clone() {
            row.push(*column.get(i).unwrap());
        }
        rows.push(row);
    }

    let mut galaxies = Vec::new();
    for (i, row) in rows.into_iter().enumerate() {
        for (j, character) in row.into_iter().enumerate() {
            if character == '#' {
                galaxies.push(Galaxy {
                    i: i as i32,
                    j: j as i32,
                });
            }
        }
    }

    galaxies
}
