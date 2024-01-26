fn main() {
    let input = include_str!("./input2.txt").trim();
    let output = process_games(input);
    dbg!(output);
}

fn process_games(input: &str) -> i32 {
    let mut power_sum = 0;
    for game in input.split('\n') {
        let game_set = game.split(':').last().unwrap();
        let game_sets: Vec<&str> = game_set.trim().split(';').collect();
        power_sum += proccess_game(game_sets);
    }

    power_sum
}

fn proccess_game(game_sets: Vec<&str>) -> i32 {
    let mut red_value = 0;
    let mut green_value = 0;
    let mut blue_value = 0;
    for game_set in game_sets {
        for draft in game_set.split(',') {
            let value = draft
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if draft.ends_with("red") && value > red_value {
                red_value = value;
            } else if draft.ends_with("green") && value > green_value {
                green_value = value;
            } else if draft.ends_with("blue") && value > blue_value {
                blue_value = value;
            }
        }
    }

    red_value * green_value * blue_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt").trim();
        let results = process_games(input);
        assert_eq!(results, 8);
    }
}
