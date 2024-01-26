fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut calibration_values_sum = 0;

    for line in input.split('\n') {
        let mut combination: Vec<char> = Vec::new();

        for character in line.chars() {
            if character.is_ascii_digit() {
                combination.push(character)
            }
        }

        if combination.is_empty() {
            continue;
        }

        if combination.len() == 1 {
            calibration_values_sum += combination[0]
                .to_string()
                .repeat(2)
                .parse::<i32>()
                .expect("Not a valid number");
            continue;
        }

        let mut value = combination[0].to_string();
        value.push(combination[combination.len() - 1]);

        calibration_values_sum += value.parse::<i32>().expect("Not a valid number");
    }

    calibration_values_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let results = part1(input);
        assert_eq!(results, 142);
    }
}
