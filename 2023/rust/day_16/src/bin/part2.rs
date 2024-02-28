use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct LightPointer {
    i: usize,
    j: usize,
    direction: Direction,
}

struct Contraption {
    layout: Vec<Vec<char>>,
    visited: HashSet<(usize, usize)>,
    visited_pointers: HashSet<LightPointer>,
}

impl Contraption {
    fn display_visited(&self) {
        let mut visited_layout =
            vec![vec!['.'; self.layout.get(0).unwrap().len()]; self.layout.len()];

        for (i, j) in self.visited.clone() {
            visited_layout[i][j] = '#';
        }

        for pointer in self.visited_pointers.clone() {
            visited_layout[pointer.i][pointer.j] = '#';
        }

        for row in visited_layout {
            println!("{}", String::from_iter(row.iter()));
        }
    }

    fn count_energized_tiles(&self) -> usize {
        let mut energized_tiles = HashSet::new();

        for pos in self.visited.clone() {
            energized_tiles.insert(pos);
        }

        for pointer in self.visited_pointers.clone() {
            energized_tiles.insert((pointer.i, pointer.j));
        }

        energized_tiles.len()
    }

    fn get_column(&self, j: usize) -> Vec<char> {
        self.layout
            .iter()
            .map(|row| *row.get(j).unwrap())
            .collect::<Vec<char>>()
    }

    fn get_row(&self, i: usize) -> Vec<char> {
        self.layout.get(i).unwrap().to_vec()
    }

    fn get_layout_slice(&self, pointer: &LightPointer) -> Vec<char> {
        match pointer.direction {
            Direction::North => self.get_column(pointer.j)[..pointer.i + 1]
                .iter()
                .rev()
                .copied()
                .collect::<Vec<char>>(),
            Direction::East => self.get_row(pointer.i)[pointer.j..].to_vec(),
            Direction::South => self.get_column(pointer.j)[pointer.i..].to_vec(),
            Direction::West => self.get_row(pointer.i)[..pointer.j + 1]
                .iter()
                .rev()
                .copied()
                .collect::<Vec<char>>(),
        }
    }

    fn get_light_reflection_pointers(
        &self,
        pointer: LightPointer,
        element: char,
    ) -> Option<Vec<LightPointer>> {
        match pointer.direction {
            Direction::West => match element {
                '|' => {
                    let mut pointers = Vec::new();

                    if pointer.i + 1 < self.layout.len() {
                        pointers.push(LightPointer {
                            i: pointer.i + 1,
                            j: pointer.j,
                            direction: Direction::South,
                        });
                    }

                    if pointer.i > 0 {
                        pointers.push(LightPointer {
                            i: pointer.i - 1,
                            j: pointer.j,
                            direction: Direction::North,
                        });
                    }

                    Some(pointers)
                }
                '\\' => {
                    if pointer.i == 0 {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i - 1,
                        j: pointer.j,
                        direction: Direction::North,
                    }])
                }
                '/' => {
                    if pointer.i + 1 > self.layout.len() {
                        return None;
                    }
                    Some(vec![LightPointer {
                        i: pointer.i + 1,
                        j: pointer.j,
                        direction: Direction::South,
                    }])
                }
                _ => None,
            },
            Direction::East => match element {
                '|' => {
                    let mut pointers = Vec::new();

                    if pointer.i + 1 < self.layout.len() {
                        pointers.push(LightPointer {
                            i: pointer.i + 1,
                            j: pointer.j,
                            direction: Direction::South,
                        });
                    }

                    if pointer.i > 0 {
                        pointers.push(LightPointer {
                            i: pointer.i - 1,
                            j: pointer.j,
                            direction: Direction::North,
                        });
                    }

                    Some(pointers)
                }
                '\\' => {
                    if pointer.i + 1 > self.layout.len() {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i + 1,
                        j: pointer.j,
                        direction: Direction::South,
                    }])
                }

                '/' => {
                    if pointer.i == 0 {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i - 1,
                        j: pointer.j,
                        direction: Direction::North,
                    }])
                }

                _ => None,
            },
            Direction::North => match element {
                '-' => {
                    let mut pointers = Vec::new();

                    if pointer.j + 1 < self.layout.get(pointer.i).unwrap().len() {
                        pointers.push(LightPointer {
                            i: pointer.i,
                            j: pointer.j + 1,
                            direction: Direction::East,
                        });
                    }

                    if pointer.j > 0 {
                        pointers.push(LightPointer {
                            i: pointer.i,
                            j: pointer.j - 1,
                            direction: Direction::West,
                        });
                    }

                    Some(pointers)
                }
                '\\' => {
                    if pointer.j == 0 {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i,
                        j: pointer.j - 1,
                        direction: Direction::West,
                    }])
                }
                '/' => {
                    if pointer.j + 1 > self.layout.get(pointer.i).unwrap().len() {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i,
                        j: pointer.j + 1,
                        direction: Direction::East,
                    }])
                }
                _ => None,
            },
            Direction::South => match element {
                '-' => {
                    let mut pointers = Vec::new();

                    if pointer.j + 1 < self.layout.get(pointer.i).unwrap().len() {
                        pointers.push(LightPointer {
                            i: pointer.i,
                            j: pointer.j + 1,
                            direction: Direction::East,
                        });
                    }

                    if pointer.j > 0 {
                        pointers.push(LightPointer {
                            i: pointer.i,
                            j: pointer.j - 1,
                            direction: Direction::West,
                        });
                    }

                    Some(pointers)
                }
                '\\' => {
                    if pointer.j + 1 > self.layout.get(pointer.i).unwrap().len() {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i,
                        j: pointer.j + 1,
                        direction: Direction::East,
                    }])
                }
                '/' => {
                    if pointer.j == 0 {
                        return None;
                    }

                    Some(vec![LightPointer {
                        i: pointer.i,
                        j: pointer.j - 1,
                        direction: Direction::West,
                    }])
                }
                _ => None,
            },
        }
    }

    fn handle_pointer(&mut self, pointer: LightPointer) -> Vec<LightPointer> {
        let layout_slice = self.get_layout_slice(&pointer);

        for (index, element) in layout_slice.iter().enumerate() {
            let (i, j) = match pointer.direction {
                Direction::North => (pointer.i - index, pointer.j),
                Direction::East => (pointer.i, pointer.j + index),
                Direction::South => (pointer.i + index, pointer.j),
                Direction::West => (pointer.i, pointer.j - index),
            };

            if *element == '.'
                || ((pointer.direction == Direction::West || pointer.direction == Direction::East)
                    && *element == '-')
                || ((pointer.direction == Direction::North
                    || pointer.direction == Direction::South)
                    && *element == '|')
            {
                self.visited.insert((i, j));
                continue;
            }

            let mirror_pointer = LightPointer {
                i,
                j,
                direction: pointer.direction,
            };

            let inserted = self.visited_pointers.insert(mirror_pointer.clone());

            if !inserted {
                return vec![];
            }

            let reflections = self.get_light_reflection_pointers(mirror_pointer, *element);

            if let Some(reflections) = reflections {
                return reflections;
            } else {
                return vec![];
            }
        }

        vec![]
    }

    fn traverse_light(&mut self, start: LightPointer) {
        let mut pointers = vec![start];

        while let Some(current_pointer) = pointers.pop() {
            let mut next_pointers = self.handle_pointer(current_pointer);
            pointers.append(&mut next_pointers);
        }
    }

    fn clear_visited(&mut self) {
        self.visited.clear();
        self.visited_pointers.clear();
    }

    fn get_best_count(&mut self) -> usize {
        let mut max = 0_usize;
        for i in 0..self.layout.len() {
            self.traverse_light(LightPointer {
                i,
                j: 0,
                direction: Direction::East,
            });
            let east_count = self.count_energized_tiles();
            if east_count > max {
                max = east_count;
            }
            self.clear_visited();

            self.traverse_light(LightPointer {
                i,
                j: self.layout.get(i).unwrap().len() - 1,
                direction: Direction::West,
            });
            let west_count = self.count_energized_tiles();
            if west_count > max {
                max = west_count;
            }
            self.clear_visited();
        }

        for j in 0..self.layout.get(0).unwrap().len() {
            self.traverse_light(LightPointer {
                i: 0,
                j,
                direction: Direction::South,
            });
            let south_count = self.count_energized_tiles();
            if south_count > max {
                max = south_count;
            }
            self.clear_visited();

            self.traverse_light(LightPointer {
                i: self.layout.len() - 1,
                j,
                direction: Direction::North,
            });
            let north_count = self.count_energized_tiles();
            if north_count > max {
                max = north_count;
            }
            self.clear_visited();
        }

        max
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let mut contraption = Contraption {
        layout: parse_input(input),
        visited: HashSet::new(),
        visited_pointers: HashSet::new(),
    };

    dbg!(contraption.get_best_count());
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split('\n')
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}
