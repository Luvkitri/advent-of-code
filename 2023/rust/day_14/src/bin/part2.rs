use std::fmt;

#[derive(Clone, PartialEq, Eq)]
struct Platform {
    elements: Vec<Vec<char>>,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rep = String::from("");
        for row in self.elements.clone() {
            rep += String::from_iter(row).as_str();
            rep += "\n";
        }
        write!(f, "{}", rep)
    }
}

impl Platform {
    fn tilt_west(&mut self) {
        let mut tilted_platform = Vec::new();
        for row in self.elements.clone() {
            let mut tilted_row = Vec::new();
            let mut empty_elements_count = 0_u32;
            for element in row {
                match element {
                    'O' => {
                        tilted_row.push(element);
                    }
                    '.' => {
                        empty_elements_count += 1;
                    }
                    '#' => {
                        for _ in 0..empty_elements_count {
                            tilted_row.push('.');
                        }
                        empty_elements_count = 0;
                        tilted_row.push(element);
                    }
                    _ => panic!("Unknown element"),
                }
            }
            for _ in 0..empty_elements_count {
                tilted_row.push('.');
            }

            tilted_platform.push(tilted_row);
        }

        self.elements = tilted_platform;
    }

    fn tilt_east(&mut self) {
        let mut tilted_platform = vec![Vec::new(); self.elements.len()];
        for (i, row) in self.elements.clone().iter().enumerate() {
            let mut empty_count = 0_usize;
            for element in row.iter().rev() {
                match element {
                    'O' => tilted_platform.get_mut(i).unwrap().insert(0, *element),
                    '.' => {
                        empty_count += 1;
                    }
                    '#' => {
                        for _ in 0..empty_count {
                            tilted_platform.get_mut(i).unwrap().insert(0, '.');
                        }
                        empty_count = 0;
                        tilted_platform.get_mut(i).unwrap().insert(0, *element);
                    }
                    _ => panic!("Unknown element"),
                }
            }
            for _ in 0..empty_count {
                tilted_platform.get_mut(i).unwrap().insert(0, '.');
            }
        }

        self.elements = tilted_platform;
    }

    fn tilt_north(&mut self) {
        let mut tilted_platform = vec![Vec::new(); self.elements.len()];
        for j in 0..self.elements.first().unwrap().len() {
            let mut empty_count = 0_usize;
            let mut last_index = 0;
            for (i, row) in self.elements.clone().iter().enumerate() {
                let element = row.get(j).unwrap();
                match element {
                    'O' => {
                        tilted_platform.get_mut(last_index).unwrap().push(*element);
                        last_index += 1;
                    }
                    '.' => {
                        empty_count += 1;
                    }
                    '#' => {
                        for index in last_index..empty_count + last_index {
                            tilted_platform.get_mut(index).unwrap().push('.');
                        }
                        tilted_platform.get_mut(i).unwrap().push(*element);
                        empty_count = 0;
                        last_index = i + 1;
                    }
                    _ => panic!("Unknown element"),
                }
            }
            for index in last_index..empty_count + last_index {
                tilted_platform.get_mut(index).unwrap().push('.');
            }
        }

        self.elements = tilted_platform;
    }

    fn tilt_south(&mut self) {
        let mut tilted_platform = vec![Vec::new(); self.elements.len()];
        for j in 0..self.elements.first().unwrap().len() {
            let mut empty_count = 0_usize;
            let mut last_index = self.elements.len() - 1;
            for (i, row) in self.elements.clone().iter().enumerate().rev() {
                let element = row.get(j).unwrap();
                match element {
                    'O' => {
                        tilted_platform.get_mut(last_index).unwrap().push(*element);
                        last_index -= 1;
                    }
                    '.' => {
                        empty_count += 1;
                    }
                    '#' => {
                        for index in (last_index - empty_count + 1..last_index + 1).rev() {
                            tilted_platform.get_mut(index).unwrap().push('.');
                        }
                        tilted_platform.get_mut(i).unwrap().push(*element);
                        empty_count = 0;
                        if i != 0 {
                            last_index = i - 1;
                        }
                    }
                    _ => panic!("Unknown element"),
                }
            }
            if empty_count > 0 {
                for index in (0..last_index + 1).rev() {
                    tilted_platform.get_mut(index).unwrap().push('.');
                }
            }
        }

        self.elements = tilted_platform;
    }

    fn measure_load(&self) -> usize {
        let mut load_value = 0;
        for (index, row) in self.elements.iter().enumerate() {
            for element in row {
                if *element == 'O' {
                    load_value += self.elements.len() - index;
                }
            }
        }
        load_value
    }
}

fn main() {
    let input = include_str!("input1.txt");
    let mut platform = parse_input(input);

    // println!("{}", platform);

    // platform.tilt_east();

    // println!("{}", platform);

    // TODO Find inner loop

    let previous_platform = platform.clone();
    for i in 0..1_00 {
        platform.tilt_north();
        platform.tilt_west();
        platform.tilt_south();
        platform.tilt_east();

        if i % 100_000 == 0 {
            println!("{}th iteration", i);
        }

        println!("{}", platform);

        if previous_platform == platform {
            break;
        }
    }

    dbg!(platform.measure_load());
}

// fn tilt_platform(platform: &Vec<Vec<PlatformElement>>) -> (Vec<Vec<PlatformElement>>, usize) {
//     let mut tilted_platform = Vec::new();
//     let mut damage_value = 0_usize;
//     for column in platform {
//         let mut tilted_column = Vec::new();
//         let mut empty_elements_count = 0_u32;
//         for element in column {
//             match element {
//                 'O' => {
//                     damage_value += column.len() - tilted_column.len();
//                     tilted_column.push(*element);
//                 }
//                 '.' => {
//                     empty_elements_count += 1;
//                 }
//                 '#' => {
//                     for _ in 0..empty_elements_count {
//                         tilted_column.push('.');
//                     }
//                     empty_elements_count = 0;
//                     tilted_column.push(*element);
//                 }
//             }
//         }
//         for _ in 0..empty_elements_count {
//             tilted_column.push('.');
//         }

//         tilted_platform.push(tilted_column);
//     }

//     (tilted_platform, damage_value)
// }

fn parse_input(input: &str) -> Platform {
    let mut platform_elements = Vec::new();
    for line in input.trim().split('\n') {
        let mut elements = Vec::new();
        for character in line.chars() {
            elements.push(character);
        }
        platform_elements.push(elements);
    }

    Platform {
        elements: platform_elements,
    }
}
