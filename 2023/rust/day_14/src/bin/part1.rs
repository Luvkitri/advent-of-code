#[derive(Copy, Clone, Debug)]
enum PlatformElement {
    RoundRock,
    CubeRock,
    Empty,
}

fn main() {
    let input = include_str!("input2.txt");
    let platform = parse_input(input);
    let (_, damage_value) = tilt_platform(&platform);
    dbg!(damage_value);
}

fn tilt_platform(platform: &Vec<Vec<PlatformElement>>) -> (Vec<Vec<PlatformElement>>, usize) {
    let mut tilted_platform = Vec::new();
    let mut damage_value = 0_usize;
    for column in platform {
        let mut tilted_column = Vec::new();
        let mut empty_elements_count = 0_u32;
        for element in column {
            match element {
                PlatformElement::RoundRock => {
                    damage_value += column.len() - tilted_column.len();
                    tilted_column.push(*element);
                }
                PlatformElement::Empty => {
                    empty_elements_count += 1;
                }
                PlatformElement::CubeRock => {
                    for _ in 0..empty_elements_count {
                        tilted_column.push(PlatformElement::Empty);
                    }
                    empty_elements_count = 0;
                    tilted_column.push(*element);
                }
            }
        }
        for _ in 0..empty_elements_count {
            tilted_column.push(PlatformElement::Empty);
        }

        tilted_platform.push(tilted_column);
    }

    (tilted_platform, damage_value)
}

fn parse_input(input: &str) -> Vec<Vec<PlatformElement>> {
    // Input is rotated while parsing

    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let mut platform = Vec::new();
    for i in 0..lines.first().unwrap().len() {
        let mut elements = Vec::new();
        for line in lines.clone() {
            let character = line.chars().nth(i).unwrap();

            elements.push(match character {
                'O' => PlatformElement::RoundRock,
                '#' => PlatformElement::CubeRock,
                '.' => PlatformElement::Empty,
                _ => panic!("Unknown character"),
            });
        }
        platform.push(elements);
    }

    platform
}
