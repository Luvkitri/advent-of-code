use std::collections::HashMap;

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

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if self.positions.len() != other.positions.len() {
            return false;
        }

        for (self_position, other_position) in self.positions.iter().zip(other.positions.iter()) {
            if self_position != other_position {
                return false;
            }
        }

        if self.value != other.value {
            return false;
        }

        true
    }
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

fn get_sum_of_part_numbers(stars: &Vec<Vec<Position>>, numbers: &HashMap<u32, Vec<Number>>) -> u32 {
    let mut sum = 0;

    for star_positions in stars {
        let mut temp_numbers: Vec<Number> = Vec::new();
        for star_position in star_positions {
            if !numbers.contains_key(&star_position.i) {
                continue;
            }

            let row_numbers = numbers.get(&star_position.i).unwrap();

            for number in row_numbers {
                if number.positions.contains(star_position) && !temp_numbers.contains(number) {
                    temp_numbers.push(number.clone());
                }
            }
        }

        if temp_numbers.len() == 2 {
            sum += temp_numbers[0].value * temp_numbers[1].value;
        }
    }

    sum
}

fn update_numbers(numbers: &mut HashMap<u32, Vec<Number>>, index: u32, number: Number) {
    if numbers.contains_key(&index) {
        numbers
            .entry(index)
            .and_modify(|row_numbers| row_numbers.push(number.clone()));
        return;
    }

    numbers.entry(index).or_insert(vec![number.clone()]);
}

fn process_engine_schematics(input: &str) -> u32 {
    // let mut passing_positions: HashSet<Position> = HashSet::new();
    let mut stars: Vec<Vec<Position>> = Vec::new();
    let lines: Vec<(usize, &str)> = input.trim().split('\n').enumerate().collect();
    let max_i = (lines.len() as i32) - 1;
    let max_j = (lines[0].1.len() as i32) - 1;

    let mut numbers: HashMap<u32, Vec<Number>> = HashMap::new();
    for (i, line) in lines {
        let mut temp_num = Vec::new();
        let mut temp_pos = Vec::new();

        for (j, character) in line.chars().enumerate() {
            if character == '.' && temp_num.is_empty() {
                continue;
            }

            if character == '.' && !temp_num.is_empty() {
                update_numbers(
                    &mut numbers,
                    i as u32,
                    Number {
                        value: temp_num.iter().collect::<String>().parse().unwrap(),
                        positions: temp_pos.clone(),
                    },
                );

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

            if character == '*' {
                let surrounding_positions = get_surrounding_positions(
                    &Position {
                        i: i as u32,
                        j: j as u32,
                    },
                    &max_i,
                    &max_j,
                );
                stars.push(surrounding_positions.clone());
            }

            if !temp_num.is_empty() {
                update_numbers(
                    &mut numbers,
                    i as u32,
                    Number {
                        value: temp_num.iter().collect::<String>().parse().unwrap(),
                        positions: temp_pos.clone(),
                    },
                );

                temp_num.clear();
                temp_pos.clear();
            }
        }

        if !temp_num.is_empty() {
            update_numbers(
                &mut numbers,
                i as u32,
                Number {
                    value: temp_num.iter().collect::<String>().parse().unwrap(),
                    positions: temp_pos.clone(),
                },
            );
            temp_num.clear();
            temp_pos.clear();
        }
    }

    get_sum_of_part_numbers(&stars, &numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./input1.txt").trim();
        let results = process_engine_schematics(input);
        assert_eq!(results, 467835);
    }

    #[test]
    fn part2() {
        let input = include_str!("./input2.txt").trim();
        let results = process_engine_schematics(input);
        assert_eq!(results, 81296995)
    }
}
