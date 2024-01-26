fn main() {
    let input = include_str!("./input2.txt").trim();
    let output = process_games(input);
    dbg!(output);
}

fn process_games(input: &str) -> u32 {
    let mut completed_games_sum = 0;
    for game in input.split('\n') {
        let (game_id, game_set) = game.split_once(':').unwrap();
        let game_id = game_id.split(' ').last().unwrap().parse::<u32>().unwrap();
        let game_sets: Vec<&str> = game_set.trim().split(';').collect();
        completed_games_sum += proccess_game(game_id, game_sets);
    }

    completed_games_sum
}

fn proccess_game(game_id: u32, game_sets: Vec<&str>) -> u32 {
    for game_set in game_sets {
        let mut red_limit: i32 = 12;
        let mut green_limit: i32 = 13;
        let mut blue_limit: i32 = 14;

        for draft in game_set.split(',') {
            let value = draft
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            if draft.ends_with("red") {
                red_limit -= value;
            } else if draft.ends_with("green") {
                green_limit -= value;
            } else if draft.ends_with("blue") {
                blue_limit -= value;
            }
        }

        if red_limit < 0 || green_limit < 0 || blue_limit < 0 {
            return 0;
        }
    }

    game_id
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
