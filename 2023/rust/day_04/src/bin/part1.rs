use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt").trim();
    let output = process_cards_winnings(input);
    dbg!(output);
}

fn process_cards_winnings(input: &str) -> u32 {
    let cards = input.split('\n');
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;

    for card in cards {
        let (winning_part_with_id, answer_part) = card.split_once('|').unwrap();
        let winning_part = winning_part_with_id.split_once(':').unwrap().1;
        let winning_numbers: Vec<u32> = re
            .find_iter(winning_part)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();
        let answer_numbers: Vec<u32> = re
            .find_iter(answer_part)
            .map(|m| m.as_str().parse::<u32>().unwrap())
            .collect();

        sum += calculate_card_points(winning_numbers, answer_numbers)
    }

    sum
}

fn calculate_card_points(winning_numbers: Vec<u32>, answer_numbers: Vec<u32>) -> u32 {
    let mut points: u32 = 0;
    for number in winning_numbers {
        if !answer_numbers.contains(&number) {
            continue;
        }

        if points < 1 {
            points = 1;
            continue;
        }

        points *= 2;
    }

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./input1.txt").trim();
        let results = process_cards_winnings(input);
        assert_eq!(results, 13);
    }

    #[test]
    fn part1() {
        let input = include_str!("./input2.txt").trim();
        let results = process_cards_winnings(input);
        assert_eq!(results, 20117)
    }
}
