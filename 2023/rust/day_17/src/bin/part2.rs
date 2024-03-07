use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;

struct CityMap {
    cities: Vec<Vec<u32>>,
    height: isize,
    width: isize,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct CityPointer {
    position: (isize, isize),
    direction: (isize, isize),
    direction_count: u32,
}

impl CityMap {
    fn get_neighbors(&self, pointer: &CityPointer) -> Vec<CityPointer> {
        let mut neighbors = Vec::new();
        let directions: Vec<(isize, isize)> = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

        let (i, j) = pointer.position;
        for direction in directions {
            if direction == (-pointer.direction.0, -pointer.direction.1) {
                continue;
            }

            let (next_i, next_j) = (direction.0 + i, direction.1 + j);
            if next_i < 0 || next_j < 0 || next_j >= self.width || next_i >= self.height {
                continue;
            }

            if direction == pointer.direction && pointer.direction_count < 10 {
                neighbors.push(CityPointer {
                    position: (next_i, next_j),
                    direction,
                    direction_count: pointer.direction_count + 1,
                });
            } else if (direction != pointer.direction && pointer.direction_count >= 4)
                || pointer.direction_count == 0
            {
                neighbors.push(CityPointer {
                    position: (next_i, next_j),
                    direction,
                    direction_count: 1,
                });
            }
        }

        neighbors
    }

    fn is_end(&self, pointer: &CityPointer) -> bool {
        if pointer.position == (self.height - 1, self.width - 1) && pointer.direction_count >= 4 {
            return true;
        }

        false
    }

    fn get_city_weight(&self, position: (isize, isize)) -> u32 {
        *self
            .cities
            .get(position.0 as usize)
            .unwrap()
            .get(position.1 as usize)
            .unwrap()
    }

    fn dijkstra_search(&self, start: &CityPointer) -> Option<u32> {
        let mut unvisited = DoublePriorityQueue::new();
        let mut visited = HashSet::new();
        unvisited.push(*start, 0);

        while let Some((current_node, distance)) = unvisited.pop_min() {
            if self.is_end(&current_node) {
                return Some(distance);
            }

            for neighbor in self.get_neighbors(&current_node) {
                let unvistied_neighbor = unvisited.get(&neighbor);

                let neighbor = if unvistied_neighbor.is_some() {
                    neighbor
                } else if visited.insert(neighbor) {
                    unvisited.push(neighbor, u32::MAX);
                    neighbor
                } else {
                    continue;
                };

                let next_distance = distance + self.get_city_weight(neighbor.position);

                if *unvisited.get(&neighbor).unwrap().1 > next_distance {
                    unvisited.change_priority(&neighbor, next_distance);
                }
            }
        }

        None
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let digits = parse_input(input);
    let city_map = CityMap {
        cities: digits.clone(),
        height: digits.len() as isize,
        width: digits.get(0).unwrap().len() as isize,
    };

    let start = CityPointer {
        position: (0, 0),
        direction: (0, 0),
        direction_count: 0,
    };

    let distance = city_map.dijkstra_search(&start);
    dbg!(distance.unwrap());
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .trim()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}
