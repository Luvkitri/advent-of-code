use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;

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
    fn get_neighbors(&self, pointer: TrailPointer) -> Vec<TrailPointer> {
        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut neighbors = Vec::new();
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

        neighbors
    }

    fn dijkstra(&self) -> u32 {
        let mut unvisited = DoublePriorityQueue::new();
        unvisited.push(self.start, 0);

        // let mut visited = HashSet::new();

        let mut max_distance = 0;

        while let Some((current_pointer, distance)) = unvisited.pop_max() {
            if current_pointer.position == self.finish && distance > max_distance {
                max_distance = distance;
            }

            for neighbor in self.get_neighbors(current_pointer) {
                let unvisited_neighbor = unvisited.get(&neighbor);

                if unvisited_neighbor.is_none() {
                    unvisited.push(neighbor, 0);
                }

                let next_distance = distance + 1;

                if *unvisited.get(&neighbor).unwrap().1 < next_distance {
                    unvisited.change_priority(&neighbor, next_distance);
                }
            }
        }

        max_distance
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let trail = parse_input(input);
    let distance = trail.dijkstra();
    dbg!(distance);
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
