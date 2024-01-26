use regex::Regex;

struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    let input = include_str!("./input2.txt");
    let races = parse_input(input);
    let mut output = 1;
    for race in races {
        output *= get_possible_wins(race);
    }
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<Race> {
    let (time_line, duration_line) = input.split_once('\n').unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let time_values = re
        .find_iter(time_line)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let duration_value = re
        .find_iter(duration_line)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    time_values
        .iter()
        .zip(duration_value.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

fn get_possible_wins(race: Race) -> u32 {
    let mut possible_wins_count = 0_u32;
    let mut button_hold_duration = 1_u32;

    while button_hold_duration < race.time {
        let distance = (race.time - button_hold_duration) * button_hold_duration;
        if distance > race.distance {
            possible_wins_count += 1;
        }
        button_hold_duration += 1;
    }

    possible_wins_count
}
