use std::collections::VecDeque;

struct Graph {
    nodes: Vec<Vec<Node>>,
    width: usize,
    height: usize,
}

impl Graph {
    fn get_neigbors(&mut self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbors = Vec::new();
        for direction in directions {
            let (next_i, next_j) = (
                direction.0 + position.0 as isize,
                direction.1 + position.1 as isize,
            );

            if next_i < 0
                || next_j < 0
                || next_j > self.width as isize
                || next_i > self.height as isize
            {
                continue;
            }

            let (next_i, next_j) = (next_i as usize, next_j as usize);
            let next_node = self.nodes[next_i][next_j];
            if next_node.character == '#' || next_node.explored {
                continue;
            }

            self.mark_node((next_i, next_j));
            neighbors.push((next_i, next_j))
        }

        neighbors
    }

    fn mark_node(&mut self, node_position: (usize, usize)) {
        self.nodes[node_position.0][node_position.1].explored =
            !self.nodes[node_position.0][node_position.1].explored;
    }

    fn unmark_nodes(&mut self, nodes_positions: Vec<(usize, usize)>) {
        for node_position in nodes_positions {
            self.mark_node(node_position);
        }
    }
}

#[derive(Clone, Copy)]
struct Node {
    character: char,
    explored: bool,
}

fn main() {
    let input = include_str!("input2.txt");
    let (starting_position, mut graph) = parse_input(input);
    bfs(&mut graph, starting_position);
}

fn bfs(graph: &mut Graph, starting_position: (usize, usize)) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([starting_position]);

    let mut step_count = 0;
    let mut garden_plots_count = 0;
    let mut layer_neighbors = Vec::new();
    let mut previous_layer_neighbors = Vec::new();
    while let Some(current_postion) = queue.pop_front() {
        let neighbors = graph.get_neigbors(current_postion);

        if queue.is_empty() {
            step_count += 1;
            layer_neighbors.extend(neighbors);
            graph.unmark_nodes(previous_layer_neighbors);
            previous_layer_neighbors = layer_neighbors.clone();
            garden_plots_count = layer_neighbors.len();
            queue.extend(layer_neighbors.clone());
            layer_neighbors.clear();
            if step_count == 64 {
                break;
            }
        } else {
            layer_neighbors.extend(neighbors)
        }
    }
    dbg!(garden_plots_count);
    dbg!(step_count);
}

fn parse_input(input: &str) -> ((usize, usize), Graph) {
    let mut rows = Vec::new();
    let mut starting_position = (0, 0);
    for (i, line) in input.trim().split('\n').enumerate() {
        let mut nodes = Vec::new();
        for (j, character) in line.trim().chars().enumerate() {
            if character == 'S' {
                starting_position = (i, j);
            }
            nodes.push(Node {
                character,
                explored: false,
            });
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
