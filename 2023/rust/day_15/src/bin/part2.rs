fn main() {
    let input = include_str!("input2.txt");
    let initialization_sequence = parse_input(input);
    let mut sum = 0;
    for step in initialization_sequence {
        sum += hash(step);
    }

    println!("{:}", sum);
}

fn hash(step: &str) -> u32 {
    let mut current_value = 0;
    for character in step.chars() {
        current_value += character as u32;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split(',').collect::<Vec<&str>>()
}
