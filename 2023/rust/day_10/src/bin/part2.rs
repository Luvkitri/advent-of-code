#[derive(PartialEq)]
enum Direction {
    Up,
    Right,
    Left,
    Down,
}

#[derive(Clone, Copy, Debug)]
struct Position {
    i: i32,
    j: i32,
}

impl Position {
    fn get_surrounding_positions(
        &self,
        max_i: i32,
        max_j: i32,
        exclude: Vec<Direction>,
    ) -> Vec<(Position, Direction)> {
        let mut surrounding_positions = Vec::new();
        let up = Position {
            i: self.i - 1,
            j: self.j,
        };
        let right = Position {
            i: self.i,
            j: self.j + 1,
        };
        let down = Position {
            i: self.i + 1,
            j: self.j,
        };
        let left = Position {
            i: self.i,
            j: self.j - 1,
        };

        if up.i >= 0 && !exclude.contains(&Direction::Up) {
            surrounding_positions.push((up, Direction::Up));
        }

        if right.j < max_j && !exclude.contains(&Direction::Right) {
            surrounding_positions.push((right, Direction::Right));
        }

        if down.i < max_i && !exclude.contains(&Direction::Down) {
            surrounding_positions.push((down, Direction::Down));
        }

        if left.j >= 0 && !exclude.contains(&Direction::Left) {
            surrounding_positions.push((left, Direction::Left));
        }

        surrounding_positions
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        if self.i == other.i && self.j == other.j {
            return true;
        }

        false
    }
}

#[derive(Clone, Copy, Debug)]
struct Pipe {
    position: Position,
    symbol: char,
}

impl PartialEq for Pipe {
    fn eq(&self, other: &Self) -> bool {
        if self.position.i == other.position.i && self.position.j == other.position.j {
            return true;
        }

        false
    }
}

impl Pipe {
    fn get_next_pipe_positions(&self, max_i: i32, max_j: i32) -> Vec<(Position, Direction)> {
        match &self.symbol {
            'S' => self
                .position
                .get_surrounding_positions(max_i, max_j, vec![]),
            '-' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Up, Direction::Down],
            ),
            '|' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Left, Direction::Right],
            ),
            'J' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Right, Direction::Down],
            ),
            'L' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Left, Direction::Down],
            ),
            '7' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Right, Direction::Up],
            ),
            'F' => self.position.get_surrounding_positions(
                max_i,
                max_j,
                vec![Direction::Left, Direction::Up],
            ),
            _ => vec![],
        }
    }

    fn is_connectable(&self, other: &Pipe, from: Direction) -> bool {
        match &self.symbol {
            'S' => match from {
                Direction::Up => {
                    if other.symbol == 'F' || other.symbol == '7' || other.symbol == '|' {
                        return true;
                    }

                    false
                }
                Direction::Right => {
                    if other.symbol == '7' || other.symbol == 'J' || other.symbol == '-' {
                        return true;
                    }

                    false
                }
                Direction::Down => {
                    if other.symbol == '|' || other.symbol == 'J' || other.symbol == 'L' {
                        return true;
                    }

                    false
                }
                Direction::Left => {
                    if other.symbol == '-' || other.symbol == 'L' || other.symbol == 'F' {
                        return true;
                    }

                    false
                }
            },
            '-' => {
                if other.symbol != '.' && other.symbol != '|' {
                    return true;
                }

                false
            }
            '|' => {
                if other.symbol != '.' && other.symbol != '-' {
                    return true;
                }

                false
            }
            _ => {
                if other.symbol != '.' {
                    return true;
                }

                false
            }
        }
    }
}

struct Plan {
    structure: Vec<Vec<Pipe>>,
    max_i: i32,
    max_j: i32,
    starting_pipe: Option<Pipe>,
}

impl Plan {
    fn get_next_pipe(&self, current: &Pipe, previous: Option<Pipe>) -> Option<Pipe> {
        let next_possible_positions = current.get_next_pipe_positions(self.max_i, self.max_j);
        for (next_possible_position, from) in next_possible_positions {
            let possible_pipe = self.structure[next_possible_position.i as usize]
                [next_possible_position.j as usize];
            if current.is_connectable(&possible_pipe, from)
                && !previous.is_some_and(|p| p == possible_pipe)
            {
                return Some(possible_pipe);
            }
        }

        None
    }

    fn retrive_pipe_loop(&self) -> PipeLoop {
        let mut pipe_loop = PipeLoop {
            elements: vec![self.starting_pipe.unwrap()],
        };
        let mut previous_pipe = None;
        let mut current_pipe = self.starting_pipe.unwrap();
        loop {
            let next_pipe = self.get_next_pipe(&current_pipe, previous_pipe);

            if next_pipe.is_some_and(|p| p.symbol == 'S') || next_pipe.is_none() {
                break;
            }

            pipe_loop.elements.push(next_pipe.unwrap());

            previous_pipe = Some(current_pipe);
            current_pipe = next_pipe.unwrap();
        }

        pipe_loop
    }

    fn count_elements_inside_loop(&self, pipe_loop: PipeLoop) -> u32 {
        let mut count = 0_u32;
        for row in self.structure.clone() {
            let mut is_inside = false;
            let mut previous_element = '.';
            for element in row {
                if pipe_loop.elements.contains(&element) {
                    match element.symbol {
                        'S' => {}
                        '|' => is_inside = !is_inside,
                        '-' => {}
                        'L' => previous_element = element.symbol,
                        'F' => previous_element = element.symbol,
                        '7' => {
                            if previous_element == 'L' {
                                is_inside = !is_inside;
                            }
                        }
                        'J' => {
                            if previous_element == 'F' {
                                is_inside = !is_inside;
                            }
                        }
                        _ => panic!("Visited '.'"),
                    }
                } else if is_inside {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Clone)]
struct PipeLoop {
    elements: Vec<Pipe>,
}

fn main() {
    let input = include_str!("input2.txt");
    let plan = parse_input(input);
    let pipe_loop = plan.retrive_pipe_loop();
    let count = plan.count_elements_inside_loop(pipe_loop);
    dbg!(count);
}

fn parse_input(input: &str) -> Plan {
    let mut plan = Plan {
        structure: Vec::new(),
        max_i: 0,
        max_j: 0,
        starting_pipe: None,
    };
    let mut line_chars = Vec::new();
    let mut starting_pipe = None;
    for (i, line) in input.trim().split('\n').enumerate() {
        line_chars.clear();
        for (j, symbol) in line.chars().enumerate() {
            let pipe = Pipe {
                position: Position {
                    i: i as i32,
                    j: j as i32,
                },
                symbol,
            };

            if symbol == 'S' {
                starting_pipe = Some(pipe);
            }

            line_chars.push(pipe);
        }
        plan.structure.push(line_chars.clone());
    }

    plan.max_i = plan.structure.len() as i32;
    plan.max_j = line_chars.len() as i32;
    plan.starting_pipe = starting_pipe;

    plan
}
