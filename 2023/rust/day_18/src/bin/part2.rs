use regex::Regex;

fn main() {
    let input = include_str!("input2.txt");
    let (coordinates, perimeter) = parse_input(input);
    let area = calculate_area(coordinates);
    dbg!(area + perimeter / 2 + 1);
}

fn calculate_area(coordinates: Vec<(i64, i64)>) -> i64 {
    coordinates
        .iter()
        .zip(coordinates.iter().cycle().skip(1))
        .fold(0, |sum, (a, b)| sum + ((a.0 * b.1) - (a.1 * b.0)))
        / 2
}

fn parse_color(color: &str) -> (char, i64) {
    let re = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    let data = re.find(color).unwrap().as_str().to_owned();
    let (length_str, direction_value_str) = data.split_at(5);
    let length = i64::from_str_radix(length_str, 16).unwrap();

    let direction = match direction_value_str.parse::<u32>() {
        Ok(0) => 'R',
        Ok(1) => 'D',
        Ok(2) => 'L',
        Ok(3) => 'U',
        _ => panic!("Unknown direction value"),
    };

    (direction, length)
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, i64) {
    let mut current_pos: (i64, i64) = (0, 0);
    let mut coordinates = Vec::new();
    let mut perimeter = 0;

    for line in input.trim().split('\n') {
        let mut line_iter = line.split(' ');
        let _skip = (
            line_iter.next().unwrap().chars().next().unwrap(),
            line_iter.next().unwrap().parse::<i64>().unwrap(),
        );
        let instructions = vec![parse_color(line_iter.next().unwrap())];

        for (direction, length) in instructions {
            perimeter += length;
            match direction {
                'R' => {
                    current_pos = (current_pos.0 + length, current_pos.1);
                }
                'L' => {
                    current_pos = (current_pos.0 - length, current_pos.1);
                }
                'U' => {
                    current_pos = (current_pos.0, current_pos.1 - length);
                }
                'D' => {
                    current_pos = (current_pos.0, current_pos.1 + length);
                }
                _ => panic!("Unknown direction"),
            };

            coordinates.push(current_pos);
        }
    }

    (coordinates, perimeter)
}
