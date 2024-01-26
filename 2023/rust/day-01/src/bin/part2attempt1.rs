use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input4.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let digits: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut calibration_values_sum = 0;
    let re = Regex::new(r"zero|one|two|three|four|five|six|seven|eight|nine|\d").unwrap();

    for line in input.split('\n') {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();

        if matches.is_empty() {
            continue;
        }

        if matches.len() == 1 && digits.contains_key(matches[0]) {
            let value = digits[matches[0]]
                .repeat(2)
                .parse::<i32>()
                .expect("Not a valid number");
            dbg!(line);
            dbg!(value);
            calibration_values_sum += value;

            continue;
        }

        if matches.len() == 1 {
            let value = matches[0]
                .repeat(2)
                .parse::<i32>()
                .expect("Not a valid number");
            dbg!(value);
            calibration_values_sum += value;
            continue;
        }

        let mut combination = "".to_owned();

        if digits.contains_key(matches[0]) {
            combination.push_str(digits[matches[0]]);
        } else {
            combination.push_str(matches[0]);
        }

        let end = matches.len() - 1;

        if digits.contains_key(matches[end]) {
            combination.push_str(digits[matches[end]]);
        } else {
            combination.push_str(matches[end]);
        }

        dbg!(combination.clone());

        calibration_values_sum += combination.parse::<i32>().expect("Not a valid number");
    }

    calibration_values_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input3.txt");
        let results = part2(input);
        assert_eq!(results, 281);
    }
}
