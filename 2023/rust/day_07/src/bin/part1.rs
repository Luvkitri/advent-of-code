use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd)]
enum CardLabel {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<CardLabel>,
    bid: u32,
    type_: Type,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.type_ as u32).cmp(&(other.type_ as u32)) {
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(&other.cards) {
                    let ordering = (*self_card as u32).cmp(&(*other_card as u32));
                    if ordering == Ordering::Equal {
                        continue;
                    }

                    return ordering;
                }

                Ordering::Equal
            }
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let mut hands = parse_input(input);
    hands.sort();
    let mut sum = 0;
    for (index, hand) in hands.clone().into_iter().enumerate() {
        sum += hand.bid * (index as u32 + 1);
        println!(
            "{:}: {hand:?} -> {}",
            (index + 1),
            (hand.bid * (index as u32 + 1))
        );
    }
    dbg!(sum);
}

fn match_card_letter_to_card_label(card_letter: char) -> CardLabel {
    match card_letter {
        'A' => CardLabel::A,
        'K' => CardLabel::K,
        'Q' => CardLabel::Q,
        'J' => CardLabel::J,
        'T' => CardLabel::T,
        '9' => CardLabel::Nine,
        '8' => CardLabel::Eight,
        '7' => CardLabel::Seven,
        '6' => CardLabel::Six,
        '5' => CardLabel::Five,
        '4' => CardLabel::Four,
        '3' => CardLabel::Three,
        '2' => CardLabel::Two,
        _ => panic!("Unknown card letter!"),
    }
}

fn match_cards_to_type(cards: &[CardLabel]) -> Type {
    let mut temp_mapping: HashMap<&CardLabel, u32> = HashMap::new();
    for card in cards.iter() {
        temp_mapping
            .entry(card)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut counts: Vec<u32> = temp_mapping.into_values().collect();
    counts.sort_unstable();

    let first_count = counts.pop();
    let second_count = counts.pop();

    if first_count.is_some_and(|count| count == 5) {
        return Type::FiveOfAKind;
    }

    if first_count.is_some_and(|count| count == 4) {
        return Type::FourOfAKind;
    }

    if first_count.is_some_and(|count| count == 3) && second_count.is_some_and(|count| count == 2) {
        return Type::FullHouse;
    }

    if first_count.is_some_and(|count| count == 3) {
        return Type::ThreeOfAKind;
    }

    if first_count.is_some_and(|count| count == 2) && second_count.is_some_and(|count| count == 2) {
        return Type::TwoPair;
    }

    if first_count.is_some_and(|count| count == 2) && second_count.is_some_and(|count| count == 1) {
        return Type::OnePair;
    }

    Type::HighCard
}
fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.trim().split('\n') {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(match_card_letter_to_card_label)
            .collect::<Vec<CardLabel>>();

        let type_ = match_cards_to_type(&cards);
        hands.push(Hand {
            cards,
            bid: bid.parse::<u32>().unwrap(),
            type_,
        })
    }

    hands
}
