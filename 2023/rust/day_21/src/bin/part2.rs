use std::collections::{HashSet, VecDeque};

struct Graph {
    nodes: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Graph {
    fn get_neighbors(
        &mut self,
        position: (isize, isize),
        visited: &HashSet<(isize, isize)>,
    ) -> Vec<(isize, isize)> {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbors = Vec::new();

        for direction in directions {
            let (next_i, next_j) = (direction.0 + position.0, direction.1 + position.1);
            let lookup_position = (
                next_i.rem_euclid(self.height as isize) as usize,
                next_j.rem_euclid(self.width as isize) as usize,
            );

            let next_character = self.nodes[lookup_position.0][lookup_position.1];

            if next_character == '#' || visited.contains(&(next_i, next_j)) {
                continue;
            }

            neighbors.push((next_i, next_j))
        }

        neighbors
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let (starting_position, mut graph) = parse_input(input);
    let value = bfs(&mut graph, starting_position, 26501365);
    dbg!(value);
}

fn bfs(graph: &mut Graph, starting_position: (isize, isize), target_step_count: isize) -> isize {
    let mut queue: VecDeque<(isize, isize)> = VecDeque::from([starting_position]);
    let mut current_layer = HashSet::new();
    let mut previous_layer = HashSet::new();

    let mut step_count = 0;
    let mut start = 0;
    let mut prev_start = 0;

    let mut values = Vec::new();
    let mut values_count = 0;
    let remainder = target_step_count % graph.width as isize;
    while let Some(current_position) = queue.pop_front() {
        let neighbors = graph.get_neighbors(current_position, &previous_layer);

        if queue.is_empty() {
            current_layer.extend(neighbors);
            let garden_plots_count = current_layer.len();
            queue.extend(current_layer.clone());
            previous_layer = current_layer.clone();
            current_layer.clear();
            step_count += 1;

            if (step_count - remainder) % graph.width as isize == 0 {
                let delta = garden_plots_count as isize - start;
                let second_difference = delta - prev_start;
                let current_values = [garden_plots_count as isize, delta, second_difference];
                values.push(current_values[values_count]);
                values_count += 1;

                println!("{:?}", values.clone());
                println!(
                    "iter {}: {}: {} {}",
                    step_count, garden_plots_count, delta, second_difference
                );

                if values.len() == 3 {
                    // 2a
                    // 3a + b
                    // a + b + c
                    let a = values[2] / 2;
                    let b = values[1] - 3 * a;
                    let c = values[0] - a - b;
                    let n = 1 + target_step_count / graph.width as isize;

                    let u = a * n.pow(2) + b * n + c;
                    return u;
                }

                start = garden_plots_count as isize;
                prev_start = delta;
            }
        } else {
            current_layer.extend(neighbors)
        }
    }
    0
}

fn parse_input(input: &str) -> ((isize, isize), Graph) {
    let mut rows = Vec::new();
    let mut starting_position = (0, 0);
    for (i, line) in input.trim().split('\n').enumerate() {
        let mut nodes = Vec::new();
        for (j, character) in line.trim().chars().enumerate() {
            if character == 'S' {
                starting_position = (i as isize, j as isize);
            }
            nodes.push(character);
        }
        rows.push(nodes);
    }

    (
        starting_position,
        Graph {
            width: rows[0].len(),
            height: rows.len(),
            nodes: rows,
        },
    )
}
