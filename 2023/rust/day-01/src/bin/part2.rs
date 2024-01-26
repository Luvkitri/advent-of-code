const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("./input4.txt");
    let output = process_lines(input);
    dbg!(output);
}

fn spelled_digits_map(spelled_digit: &str) -> Option<char> {
    match spelled_digit {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn process_lines(input: &str) -> u32 {
    let mut combinations_sum = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }
        combinations_sum += find_number(line)
    }
    combinations_sum
}

fn find_number(line: &str) -> u32 {
    let digits = find_all_digits(line);

    if digits.len() == 1 {
        return format!("{}{}", digits[0], digits[0])
            .parse::<u32>()
            .expect("Not a valid number.");
    }

    format!("{}{}", digits[0], digits[digits.len() - 1])
        .parse::<u32>()
        .expect("Not a valid number.")
}

fn find_all_digits(line: &str) -> Vec<u32> {
    (0..line.len())
        .filter_map(|index| {
            let mut result = char::default();
            let fragment = &line[index..];
            for spelled_digit in SPELLED_DIGITS {
                if fragment.starts_with(spelled_digit) {
                    result = spelled_digits_map(spelled_digit).unwrap();
                }
            }

            let first_character = fragment.chars().next().unwrap();
            if first_character.is_ascii_digit() {
                result = first_character;
            }

            result.to_digit(10)
        })
        .collect::<Vec<u32>>()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input3.txt");
        let results = process_lines(input);
        assert_eq!(results, 281);
    }
}
