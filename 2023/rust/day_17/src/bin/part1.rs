enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

struct CruciblePointer {
    i: usize,
    j: usize,
    direction: Direction,
}

struct CityMap {
    cities: Vec<Vec<u32>>,
}

impl CityMap {
    fn get_column(&self, j: usize) -> Vec<u32> {
        self.cities
            .iter()
            .map(|row| *row.get(j).unwrap())
            .collect::<Vec<u32>>()
    }

    fn get_row(&self, i: usize) -> Vec<u32> {
        self.cities.get(i).unwrap().to_vec()
    }

    fn get_negative_offset(index: usize) -> usize {
        // -3 -> 0
        // -2 -> 1
        // -1 - > 2
        let difference = index as isize - 3;

        if difference < 0 {
            return (difference + 3) as usize;
        }

        index
    }

    fn get_top_path(&self, i: usize, j: usize) -> Vec<u32> {
        self.get_column(j)[Self::get_negative_offset(i)..i]
            .iter()
            .rev()
            .copied()
            .collect::<Vec<u32>>()
    }

    fn get_right_path(&self, i: usize, j: usize) -> Vec<u32> {
        self.get_row(i)[j + 1..j + 3].to_vec()
    }

    fn get_bottom_path(&self, i: usize, j: usize) -> Vec<u32> {
        self.get_column(j)[i + 1..i + 3].to_vec()
    }

    fn get_left_path(&self, i: usize, j: usize) -> Vec<u32> {
        self.get_row(i)[Self::get_negative_offset(j)..j]
            .iter()
            .rev()
            .copied()
            .collect::<Vec<u32>>()
    }

    fn get_possible_paths(&self, pointer: CruciblePointer) -> Vec<Vec<u32>> {
        let (i, j) = (pointer.i, pointer.j);

        match pointer.direction {
            Direction::Top => vec![
                self.get_top_path(i, j),
                self.get_top_path(i, j),
                self.get_left_path(i, j),
            ],
            Direction::Right => vec![
                self.get_bottom_path(i, j),
                self.get_right_path(i, j),
                self.get_top_path(i, j),
            ],
            Direction::Bottom => vec![
                self.get_bottom_path(i, j),
                self.get_right_path(i, j),
                self.get_left_path(i, j),
            ],
            Direction::Left => vec![
                self.get_bottom_path(i, j),
                self.get_right_path(i, j),
                self.get_top_path(i, j),
            ],
        }
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let digits = parse_input(input);
    let city_map = CityMap { cities: digits };
    let possible_paths = city_map.get_possible_paths(CruciblePointer {
        i: 0,
        j: 0,
        direction: Direction::Right,
    });

    dbg!(possible_paths);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
