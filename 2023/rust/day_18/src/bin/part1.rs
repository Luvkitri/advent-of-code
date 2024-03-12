use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq)]
struct DigElement {
    character: char,
    color: Option<String>,
    is_wall: bool,
}

struct DigPlan {
    layout: Vec<Vec<DigElement>>,
    width: isize,
    height: isize,
}

struct DigPointer {
    direction: (isize, isize),
    length: u32,
    color: String,
}

impl DigPlan {
    fn generate_empty_layout(width: isize, height: isize) -> Vec<Vec<DigElement>> {
        let mut layout = Vec::new();

        for _ in 0..height {
            layout.push(vec![
                DigElement {
                    character: '.',
                    color: None,
                    is_wall: false
                };
                width as usize
            ]);
        }

        layout
    }

    fn new(width: isize, height: isize) -> DigPlan {
        DigPlan {
            width,
            height,
            layout: Self::generate_empty_layout(width, height),
        }
    }

    fn create_trenches(
        &mut self,
        dig_pointers: &Vec<DigPointer>,
        starting_position: (isize, isize),
    ) {
        let mut current_pos: (isize, isize) = starting_position;
        self.layout[current_pos.0 as usize][current_pos.1 as usize] = DigElement {
            character: '#',
            color: Some("(#000000)".to_owned()),
            is_wall: true,
        };
        for dig_pointer in dig_pointers {
            for _ in 0..dig_pointer.length {
                current_pos = (
                    current_pos.0 + dig_pointer.direction.0,
                    current_pos.1 + dig_pointer.direction.1,
                );
                self.layout[current_pos.0 as usize][current_pos.1 as usize] = DigElement {
                    character: '#',
                    color: Some(dig_pointer.color.clone()),
                    is_wall: true,
                };
            }
        }
    }

    fn flood_fill(&mut self, position: (isize, isize)) {
        let mut queue = VecDeque::from([position]);

        let (i, j) = position;
        let current = &mut self.layout[i as usize][j as usize];

        if current.is_wall || (current.color.is_some() && current.character == '#') {
            return;
        }

        current.character = '#';
        current.color = Some("(#000000)".to_owned());

        let dir: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        while let Some(current_position) = queue.pop_front() {
            for direction in dir {
                let next_position = (
                    current_position.0 + direction.0,
                    current_position.1 + direction.1,
                );

                if next_position.0 < 0
                    || next_position.1 < 0
                    || next_position.0 >= self.height
                    || next_position.1 >= self.width
                {
                    continue;
                }

                let next = &mut self.layout[next_position.0 as usize][next_position.1 as usize];

                if next.is_wall || (next.color.is_some() && next.character == '#') {
                    continue;
                }

                next.character = '#';
                next.color = Some("(#000000)".to_owned());
                queue.push_back(next_position);
            }
        }
    }

    fn calculate_area(&self) -> u32 {
        let mut count = 0_u32;
        for row in self.layout.clone() {
            for element in row {
                if element.character == '#' {
                    count += 1;
                }
            }
        }
        count
    }

    fn _display(&self) {
        for row in self.layout.clone() {
            for element in row {
                print!("{}", element.character);
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let (width, height, dig_pointers, starting_position) = parse_input(input);
    let mut dig_plan = DigPlan::new(width, height);
    // dig_plan._display();
    dig_plan.create_trenches(
        &dig_pointers,
        (starting_position.0 as isize, starting_position.1 as isize),
    );
    // dig_plan._display();
    dig_plan.flood_fill((
        starting_position.0 as isize + 1,
        starting_position.1 as isize + 1,
    ));
    // dig_plan._display();
    let area = dig_plan.calculate_area();
    dbg!(area);
}

fn match_direction(direction_letter: char) -> (isize, isize) {
    match direction_letter {
        'R' => (0, 1),
        'L' => (0, -1),
        'D' => (1, 0),
        'U' => (-1, 0),
        _ => panic!("Unknown direction letter"),
    }
}

fn parse_input(input: &str) -> (isize, isize, Vec<DigPointer>, (i32, i32)) {
    let mut starting_pos = (0, 0);
    let mut max_up = 0;
    let mut max_down = 0;
    let mut max_right = 0;
    let mut max_left = 0;

    let mut dig_pointers = Vec::new();

    for line in input.trim().split('\n') {
        let mut line_iter = line.split(' ');
        let direction = line_iter.next().unwrap().chars().next().unwrap();
        let length = line_iter.next().unwrap().parse::<i32>().unwrap();
        let color = line_iter.next().unwrap();

        match direction {
            'R' => {
                starting_pos = (starting_pos.0, starting_pos.1 + length);
                if starting_pos.1 > max_right {
                    max_right = starting_pos.1;
                }
            }
            'L' => {
                starting_pos = (starting_pos.0, starting_pos.1 - length);
                if starting_pos.1 < max_left {
                    max_left = starting_pos.1;
                }
            }
            'U' => {
                starting_pos = (starting_pos.0 - length, starting_pos.1);
                if starting_pos.0 < max_up {
                    max_up = starting_pos.0;
                }
            }
            'D' => {
                starting_pos = (starting_pos.0 + length, starting_pos.1);
                if starting_pos.0 > max_down {
                    max_down = starting_pos.0;
                }
            }
            _ => panic!("Unknown direction"),
        }

        dig_pointers.push(DigPointer {
            direction: match_direction(direction),
            length: length as u32,
            color: color.to_owned(),
        })
    }

    starting_pos = (max_up.abs(), max_left.abs());
    let width = max_left.abs() + max_right;
    let height = max_up.abs() + max_down;

    (
        width as isize + 1,
        height as isize + 1,
        dig_pointers,
        starting_pos,
    )
}
