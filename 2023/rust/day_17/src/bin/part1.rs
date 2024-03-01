use ::std::collections::HashMap;
use ::std::collections::HashSet;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

struct CruciblePointer {
    i: usize,
    j: usize,
    direction: Direction,
}

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd)]
struct CityNode {
    id: (usize, usize),
    distance: u32,
    weight: u32,
    visited: bool,
}

// struct CityMap {
//     cities: Vec<Vec<u32>>,
// }

// impl CityMap {
//     fn get_column(&self, j: usize) -> Vec<u32> {
//         self.cities
//             .iter()
//             .map(|row| *row.get(j).unwrap())
//             .collect::<Vec<u32>>()
//     }

//     fn get_row(&self, i: usize) -> Vec<u32> {
//         self.cities.get(i).unwrap().to_vec()
//     }

//     fn get_negative_offset(index: usize) -> usize {
//         // -3 -> 0
//         // -2 -> 1
//         // -1 - > 2
//         let difference = index as isize - 3;

//         if difference < 0 {
//             return (difference + 3) as usize;
//         }

//         index
//     }

//     fn get_top_path(&self, i: usize, j: usize) -> Vec<u32> {
//         self.get_column(j)[Self::get_negative_offset(i)..i]
//             .iter()
//             .rev()
//             .copied()
//             .collect::<Vec<u32>>()
//     }

//     fn get_right_path(&self, i: usize, j: usize) -> Vec<u32> {
//         self.get_row(i)[j + 1..j + 3].to_vec()
//     }

//     fn get_bottom_path(&self, i: usize, j: usize) -> Vec<u32> {
//         self.get_column(j)[i + 1..i + 3].to_vec()
//     }

//     fn get_left_path(&self, i: usize, j: usize) -> Vec<u32> {
//         self.get_row(i)[Self::get_negative_offset(j)..j]
//             .iter()
//             .rev()
//             .copied()
//             .collect::<Vec<u32>>()
//     }

//     fn get_possible_paths(&self, pointer: CruciblePointer) -> Vec<Vec<u32>> {
//         let (i, j) = (pointer.i, pointer.j);

//         match pointer.direction {
//             Direction::Top => vec![
//                 self.get_top_path(i, j),
//                 self.get_top_path(i, j),
//                 self.get_left_path(i, j),
//             ],
//             Direction::Right => vec![
//                 self.get_bottom_path(i, j),
//                 self.get_right_path(i, j),
//                 self.get_top_path(i, j),
//             ],
//             Direction::Bottom => vec![
//                 self.get_bottom_path(i, j),
//                 self.get_right_path(i, j),
//                 self.get_left_path(i, j),
//             ],
//             Direction::Left => vec![
//                 self.get_bottom_path(i, j),
//                 self.get_right_path(i, j),
//                 self.get_top_path(i, j),
//             ],
//         }
//     }
// }
#[derive(Clone)]
struct CityGraph {
    nodes_map: HashMap<(usize, usize), CityNode>,
}

impl CityGraph {
    fn next_node(&self) -> Option<&mut CityNode> {
        let mut nodes = self
            .nodes_map
            .values()
            .filter(|node| !node.visited)
            .collect::<Vec<&CityNode>>();
        nodes.sort_by(|a, b| a.distance.cmp(&b.distance));
        nodes.get_mut(0)
    }
}

struct CityMap {
    cities: Vec<Vec<u32>>,
}

impl CityMap {
    fn get_neighbors(&self, position: (usize, usize)) -> HashMap<Direction, (usize, usize)> {
        let (i, j) = position;
        let mut neighbors = HashMap::new();
        if i > 0 {
            neighbors.insert(Direction::Top, (i - 1, j));
        }

        if j > 0 {
            neighbors.insert(Direction::Left, (i, j - 1));
        }

        if i < self.cities.len() - 1 {
            neighbors.insert(Direction::Bottom, (i + 1, j));
        }

        if j < self.cities.get(0).unwrap().len() {
            neighbors.insert(Direction::Right, (i, j + 1));
        }
        neighbors
    }

    fn get_nodes(&self) -> HashMap<(usize, usize), CityNode> {
        let mut nodes = HashMap::new();
        for (i, row) in self.cities.iter().enumerate() {
            for (j, weight) in row.iter().enumerate() {
                let distance = if i == 0 && j == 0 { *weight } else { u32::MAX };
                nodes.insert(
                    (i, j),
                    CityNode {
                        id: (i, j),
                        distance,
                        weight: *weight,
                        visited: false,
                    },
                );
            }
        }
        nodes
    }

    fn get_opposite_direction(direction: Direction) -> Direction {
        match direction {
            Direction::Top => Direction::Bottom,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
        }
    }

    fn match_position_to_direction(position: (isize, isize)) -> Direction {
        match position {
            (0, 1) => Direction::Right,
            (-1, 0) => Direction::Top,
            (0, -1) => Direction::Left,
            (1, 0) => Direction::Bottom,
            _ => panic!("Unknown directional position"),
        }
    }

    fn check_node(&self, node: &CityNode) -> bool {
        if node.id
            == (
                self.cities.len(),
                self.cities.get(self.cities.len() - 1).unwrap().len(),
            )
        {
            return true;
        }

        false
    }

    // Based on Dijkstra algorithm
    fn find_shortest_path(&self) -> Option<CityNode> {
        let mut graph = CityGraph {
            nodes_map: self.get_nodes(),
        };

        let mut previous_direction = Direction::Right;
        let mut previous_node: Option<CityNode> = None;

        while let Some(current_node) = graph.next_node() {
            if let Some(previous_node) = previous_node {
                previous_direction = Self::match_position_to_direction((
                    current_node.id.0 as isize - previous_node.id.0 as isize,
                    current_node.id.1 as isize - previous_node.id.1 as isize,
                ))
            }

            for (direction, neighbor) in self.get_neighbors(current_node.id) {
                let node = graph.nodes_map.get_mut(&neighbor).unwrap();

                if node.visited || direction == Self::get_opposite_direction(previous_direction) {
                    continue;
                }

                let next_distance = current_node.distance + node.weight;
                if next_distance > node.distance {
                    node.distance = next_distance;
                }
            }

            current_node.visited = true;

            if self.check_node(current_node) {
                return Some(current_node.clone());
            }

            previous_node = Some(current_node.clone());
        }

        None
    }
}

// TODO rewrite it somehow... Queue? For sure there is a simple way

fn main() {
    let input = include_str!("input1.txt");
    let digits = parse_input(input);
    let city_map = CityMap { cities: digits };
    let destination = city_map.find_shortest_path();
    dbg!(destination.unwrap().distance);
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
