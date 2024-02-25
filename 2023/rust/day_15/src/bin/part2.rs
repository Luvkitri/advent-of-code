use ::std::collections::VecDeque;

#[derive(Clone)]
struct Lense {
    label: String,
    focal_length: Option<usize>,
}

impl PartialEq for Lense {
    fn eq(&self, other: &Self) -> bool {
        if self.label == other.label {
            return true;
        }

        false
    }
}

#[derive(Clone)]
struct CBox {
    lenses: VecDeque<Lense>,
}

impl CBox {
    fn add_lense(&mut self, lense: Lense) {
        let lense_existing_position = self.get_lense_postion(&lense);

        if let Some(index) = lense_existing_position {
            self.lenses[index] = lense;
            return;
        }

        self.lenses.push_back(lense);
    }

    fn get_lense_postion(&self, lense: &Lense) -> Option<usize> {
        for (index, boxed_lense) in self.lenses.iter().enumerate() {
            if boxed_lense == lense {
                return Some(index);
            }
        }

        None
    }

    fn remove_lense(&mut self, lense: Lense) {
        let lense_existing_position = self.get_lense_postion(&lense);

        if let Some(index) = lense_existing_position {
            self.lenses.remove(index);
        }
    }

    fn validate_lenses(&self, box_number: usize) -> usize {
        let mut sum = 0;
        for (index, lense) in self.lenses.iter().enumerate() {
            sum += (1 + box_number) * (index + 1) * lense.focal_length.unwrap();
        }
        sum
    }
}

fn main() {
    let mut boxes = vec![
        CBox {
            lenses: VecDeque::new()
        };
        256
    ];

    let input = include_str!("input2.txt");
    let initialization_sequence = parse_input(input);

    for lense in initialization_sequence {
        let cbox = boxes.get_mut(hash(&lense.label)).unwrap();

        if lense.focal_length.is_none() {
            cbox.remove_lense(lense);
            continue;
        }

        cbox.add_lense(lense);
    }

    let mut sum = 0;
    for (index, cbox) in boxes.iter().enumerate() {
        sum += cbox.validate_lenses(index);
    }

    dbg!(sum);
}

fn hash(step: &str) -> usize {
    let mut current_value = 0;
    for character in step.chars() {
        current_value += character as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn parse_input(input: &str) -> Vec<Lense> {
    input
        .trim()
        .split(',')
        .map(|step| {
            if step.contains('=') {
                let (label, focal_length) = step.split_once('=').unwrap();
                return Lense {
                    label: label.to_owned(),
                    focal_length: Some(focal_length.parse::<usize>().unwrap()),
                };
            }

            let mut label = step.to_owned();
            label.pop();

            return Lense {
                label,
                focal_length: None,
            };
        })
        .collect::<Vec<Lense>>()
}
