use regex::Regex;

struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let input = include_str!("./input2.txt");
    let race = parse_input(input);
    let output = get_possible_wins(race);

    dbg!(output);
}

fn parse_input(input: &str) -> Race {
    let (time_line, distance_line) = input.split_once('\n').unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let time_value = re
        .find_iter(time_line)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let distance_value = re
        .find_iter(distance_line)
        .map(|m| m.as_str())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    Race {
        time: time_value,
        distance: distance_value,
    }
}

fn get_possible_wins(race: Race) -> u64 {
    let mut possible_wins_count = 0_u64;
    let mut button_hold_duration = 1_u64;

    while button_hold_duration < race.time {
        let distance = (race.time - button_hold_duration) * button_hold_duration;
        if distance > race.distance {
            possible_wins_count += 1;
        }
        button_hold_duration += 1;
    }

    possible_wins_count
}
