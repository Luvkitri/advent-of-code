use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    i: u32,
    j: u32,
}

#[derive(Clone)]
struct Number {
    value: u32,
    positions: Vec<Position>,
}

fn main() {
    let input = include_str!("./input2.txt").trim();
    let output = process_engine_schematics(input);
    dbg!(output);
}

fn get_surrounding_positions(
    starting_position: &Position,
    max_i: &i32,
    max_j: &i32,
) -> Vec<Position> {
    let mut positions: Vec<Position> = Vec::new();
    let range: [i32; 3] = [-1, 0, 1];

    for i in range {
        let surrounding_i = starting_position.i as i32 + i;

        if surrounding_i > *max_i || surrounding_i < 0 {
            continue;
        }

        for j in range {
            let surrounding_j = starting_position.j as i32 + j;

            if surrounding_j > *max_j || surrounding_j < 0 {
                continue;
            }

            positions.push(Position {
                i: surrounding_i as u32,
                j: surrounding_j as u32,
            })
        }
    }

    positions
}

fn get_sum_of_part_numbers(passing_positions: &HashSet<Position>, numbers: &Vec<Number>) -> u32 {
    let mut sum = 0;
    for number in numbers {
        for position in &number.positions {
            if passing_positions.contains(position) {
                sum += number.value;
                break;
            }
        }
    }

    sum
}

fn process_engine_schematics(input: &str) -> u32 {
    let mut passing_positions: HashSet<Position> = HashSet::new();
    let lines: Vec<(usize, &str)> = input.trim().split('\n').enumerate().collect();
    let max_i = (lines.len() as i32) - 1;
    let max_j = (lines[0].1.len() as i32) - 1;

    let mut numbers = Vec::new();
    for (i, line) in lines {
        let mut temp_num = Vec::new();
        let mut temp_pos = Vec::new();

        for (j, character) in line.chars().enumerate() {
            if character == '.' && temp_num.is_empty() {
                continue;
            }

            if character == '.' && !temp_num.is_empty() {
                numbers.push(Number {
                    value: temp_num.iter().collect::<String>().parse().unwrap(),
                    positions: temp_pos.clone(),
                });
                temp_num.clear();
                temp_pos.clear();
                continue;
            }

            if character.is_ascii_digit() {
                temp_num.push(character);
                temp_pos.push(Position {
                    i: i as u32,
                    j: j as u32,
                });
                continue;
            }

            let surrounding_positions = get_surrounding_positions(
                &Position {
                    i: i as u32,
                    j: j as u32,
                },
                &max_i,
                &max_j,
            );

            for surrounding_position in surrounding_positions {
                passing_positions.insert(surrounding_position);
            }

            if !temp_num.is_empty() {
                numbers.push(Number {
                    value: temp_num.iter().collect::<String>().parse().unwrap(),
                    positions: temp_pos.clone(),
                });
                temp_num.clear();
                temp_pos.clear();
            }
        }

        if !temp_num.is_empty() {
            numbers.push(Number {
                value: temp_num.iter().collect::<String>().parse().unwrap(),
                positions: temp_pos.clone(),
            });
            temp_num.clear();
            temp_pos.clear();
        }
    }

    get_sum_of_part_numbers(&passing_positions, &numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt").trim();
        let results = process_engine_schematics(input);
        assert_eq!(results, 4361);
    }
}
