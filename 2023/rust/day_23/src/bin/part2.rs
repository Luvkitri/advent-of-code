use std::collections::{HashMap, HashSet};

struct Trail {
    nodes: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: TrailPointer,
    finish: (usize, usize),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
struct TrailPointer {
    position: (usize, usize),
    direction: (isize, isize),
}

impl Trail {
    fn get_neighbors(&self, pointer: TrailPointer) -> (Vec<TrailPointer>, u32) {
        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut neighbors = Vec::new();
        let mut intersection_count = 0;
        for direction in directions {
            if direction == (-pointer.direction.0, -pointer.direction.1) {
                continue;
            }

            let (neighbor_i, neighbor_j) = (
                pointer.position.0 as isize + direction.0,
                pointer.position.1 as isize + direction.1,
            );

            if neighbor_i < 0
                || neighbor_j < 0
                || neighbor_i >= self.height as isize
                || neighbor_j >= self.width as isize
            {
                continue;
            }

            let neighbor_character = self.nodes[neighbor_i as usize][neighbor_j as usize];
            if neighbor_character == '>'
                || neighbor_character == '<'
                || neighbor_character == '^'
                || neighbor_character == 'v'
            {
                intersection_count += 1;
            }

            if neighbor_character == '#'
                || (neighbor_character == '>' && direction == (0, -1))
                || (neighbor_character == '<' && direction == (0, 1))
                || (neighbor_character == '^' && direction == (1, 0))
                || (neighbor_character == 'v' && direction == (-1, 0))
            {
                continue;
            }

            neighbors.push(TrailPointer {
                position: (neighbor_i as usize, neighbor_j as usize),
                direction,
            });
        }

        (neighbors, intersection_count)
    }

    fn find_longest_path(&self) {
        // Find all verticies (between intersections)
        // (current position, current distance, last intersection position, distance to last intersection)
        let mut stack = vec![(self.start, 0, self.start.position, 0)];
        let mut verticies: HashMap<(usize, usize), HashSet<((usize, usize), i64)>> = HashMap::new();

        while let Some((
            current_pointer,
            current_distance,
            last_intersection_position,
            distance_to_last_intersection,
        )) = stack.pop()
        {
            if current_pointer.position == self.finish {
                verticies
                    .entry(last_intersection_position)
                    .or_default()
                    .insert((
                        self.finish,
                        current_distance - distance_to_last_intersection,
                    ));
                continue;
            }

            let (neighbors, intersection_count) = self.get_neighbors(current_pointer);

            let mut next_last_intersection = last_intersection_position;
            let mut next_distance_to_last_intersection = distance_to_last_intersection;

            // Intersection
            if intersection_count > 1 {
                verticies
                    .entry(current_pointer.position)
                    .or_default()
                    .insert((
                        last_intersection_position,
                        current_distance - distance_to_last_intersection,
                    ));
                verticies
                    .entry(last_intersection_position)
                    .or_default()
                    .insert((
                        current_pointer.position,
                        current_distance - distance_to_last_intersection,
                    ));

                next_last_intersection = current_pointer.position;
                next_distance_to_last_intersection = current_distance;
            }

            for neigbor in neighbors {
                stack.push((
                    neigbor,
                    current_distance + 1,
                    next_last_intersection,
                    next_distance_to_last_intersection,
                ));
            }
        }

        let mut stack = vec![(self.start.position, 0, HashSet::new())];
        let mut max_distance = 0;

        while let Some((current_position, distance, mut visited)) = stack.pop() {
            if current_position == self.finish {
                max_distance = max_distance.max(distance);
                continue;
            }

            if !visited.insert(current_position) {
                continue;
            }

            for (next_position, next_distance) in verticies.get(&current_position).unwrap().iter() {
                stack.push((*next_position, next_distance + distance, visited.clone()));
            }
        }

        dbg!(max_distance);
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let trail = parse_input(input);
    trail.find_longest_path();
}

fn parse_input(input: &str) -> Trail {
    let mut start = (0, 0);
    let mut finish = (0, 0);
    let mut nodes = Vec::new();

    let lines = input.trim().split('\n').collect::<Vec<_>>();
    let height = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let mut characters = Vec::new();
        for (j, character) in line.trim().chars().enumerate() {
            if i == 0 && character == '.' {
                start = (i, j);
            }

            if i == height - 1 && character == '.' {
                finish = (i, j);
            }

            characters.push(character);
        }
        nodes.push(characters);
    }

    Trail {
        width: nodes.first().unwrap().len(),
        height,
        nodes,
        start: TrailPointer {
            position: start,
            direction: (1, 0),
        },
        finish,
    }
}
