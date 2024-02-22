use std::fmt;

#[derive(Clone, PartialEq, Eq, Debug)]
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
                        last_index = last_index.saturating_sub(1)
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

    fn find_inner_loop(&mut self) -> Option<(usize, Vec<Platform>)> {
        let mut previous_platforms = vec![self.clone()];
        for i in 0..1_000_000_000_usize {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            if previous_platforms.contains(self) {
                let begining_of_loop = previous_platforms.iter().position(|p| *p == *self).unwrap();
                return Some((i, previous_platforms.split_off(begining_of_loop)));
            }

            previous_platforms.push(self.clone())
        }

        None
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let mut platform = parse_input(input);
    let (iter_index, inner_loop) = platform.find_inner_loop().unwrap();

    let number_of_iter_left = 1_000_000_000 - iter_index;
    let last_element_index = number_of_iter_left % inner_loop.len() - 1;
    println!("{:}", last_element_index);
    let platform = inner_loop.get(last_element_index).unwrap();
    println!("{:}", platform.measure_load());
}

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
