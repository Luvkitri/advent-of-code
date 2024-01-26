use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt").trim();
    let output = process_cards_winnings(input);
    dbg!(output);
}

fn update_scratchcards(scratchcards: &mut HashMap<u32, u32>, card_id: u32, increment: u32) {
    scratchcards
        .entry(card_id)
        .and_modify(|value| *value += increment)
        .or_insert(increment);
}

fn process_cards_winnings(input: &str) -> u32 {
    let cards = input.split('\n');
    let re = Regex::new(r"\d+").unwrap();

    let mut scratchcards: HashMap<u32, u32> = HashMap::new();

    for (index, card) in cards.enumerate() {
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

        let card_id = index as u32 + 1;
        update_scratchcards(&mut scratchcards, card_id, 1);

        let copies = get_scratchcard_copies(winning_numbers, answer_numbers, card_id);

        for copy in copies {
            let increment = *scratchcards.get(&card_id).unwrap();
            update_scratchcards(&mut scratchcards, copy, increment)
        }
    }

    scratchcards.values().sum()
}

fn get_scratchcard_copies(
    winning_numbers: Vec<u32>,
    answer_numbers: Vec<u32>,
    card_id: u32,
) -> Vec<u32> {
    let mut copies = Vec::new();
    let mut index = 1;

    for number in winning_numbers {
        if answer_numbers.contains(&number) {
            copies.push(card_id + index);
            index += 1;
        }
    }

    copies
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("./input1.txt").trim();
        let results = process_cards_winnings(input);
        assert_eq!(results, 30);
    }

    #[test]
    fn part2() {
        let input = include_str!("./input2.txt").trim();
        let results = process_cards_winnings(input);
        assert_eq!(results, 13768818);
    }
}
