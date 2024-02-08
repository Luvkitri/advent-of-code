fn main() {
    let input = include_str!("input2.txt");
    let sequences = parse_input(input);
    let answer = analyze_oasis_report(&sequences);
    dbg!(answer);
}

fn analyze_oasis_report(sequences: &[Vec<i64>]) -> i64 {
    let mut history_value_sum = 0_i64;
    for sequence in sequences {
        let subsequences = get_all_subsequences(sequence);
        history_value_sum += get_history_value(&subsequences);
    }
    history_value_sum
}

fn get_history_value(subsequences: &[Vec<i64>]) -> i64 {
    let mut history_value = 0_i64;
    for i in (0..subsequences.len()).rev() {
        history_value = subsequences.get(i).unwrap().first().unwrap() - history_value;
    }
    history_value
}

fn get_all_subsequences(sequence: &[i64]) -> Vec<Vec<i64>> {
    let mut subsequences = Vec::new();
    let mut current_subsequence = sequence.to_owned();
    loop {
        subsequences.push(current_subsequence.clone());
        current_subsequence = get_subsequence(&current_subsequence);
        if *current_subsequence.last().unwrap() == 0 {
            break;
        }
    }
    subsequences
}

fn get_subsequence(sequence: &Vec<i64>) -> Vec<i64> {
    let mut subsequence = Vec::new();
    for i in 0..(sequence.len() - 1) {
        subsequence.push(sequence.get(i + 1).unwrap() - sequence.get(i).unwrap());
    }
    subsequence
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
    for line in input.trim().split('\n') {
        sequences.push(
            line.trim()
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>(),
        );
    }
    sequences
}
