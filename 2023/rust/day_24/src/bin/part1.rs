use std::ops::RangeInclusive;

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    position: (f64, f64, f64),
    velocity: (f64, f64, f64),
    m: f64,
    b: f64,
}

impl Hailstone {
    fn new(position: (f64, f64, f64), velocity: (f64, f64, f64)) -> Hailstone {
        let m = ((position.1 + velocity.1) - position.1) / ((position.0 + velocity.0) - position.0);
        let b = position.1 - (m * position.0);

        Self {
            position,
            velocity,
            m,
            b,
        }
    }

    fn is_point_in_the_past(&self, point: (f64, f64)) -> bool {
        let next_position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );

        if (self.velocity.0 < 0.0 && point.0 > next_position.0
            || self.velocity.0 > 0.0 && point.0 < next_position.0)
            || (self.velocity.1 < 0.0 && point.1 > next_position.1
                || self.velocity.1 > 1.0 && point.1 < next_position.1)
        {
            return true;
        }

        false
    }

    fn get_intersection_xy(&self, other: &Self) -> Option<(f64, f64)> {
        // Paths are parallel
        if self.m == other.m {
            return None;
        }

        let x = (self.b - other.b) / (other.m - self.m);
        let y = self.m * x + self.b;
        let intersection_position = (x, y);

        if self.is_point_in_the_past(intersection_position)
            || other.is_point_in_the_past(intersection_position)
        {
            return None;
        }

        Some(intersection_position)
    }
}

fn main() {
    let input = include_str!("input2.txt");
    let hailstones = parse_input(input);
    let valid_intersection_count =
        count_intersections(&hailstones, 200000000000000.0..=400000000000000.0);

    dbg!(valid_intersection_count);
}

fn count_intersections(hailstones: &[Hailstone], valid_range: RangeInclusive<f64>) -> u32 {
    let mut valid_intersections_count = 0;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let intersection_position = hailstones[i].get_intersection_xy(&hailstones[j]);

            if intersection_position
                .is_some_and(|(x, y)| valid_range.contains(&x) && valid_range.contains(&y))
            {
                // println!("Hailstone A: {hailstone:?}\nHailstone B: {next_hailstone:?}\ninteresects at: {intersection_position:?}\n");
                valid_intersections_count += 1;
            }
        }
    }

    valid_intersections_count
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    let mut hailstones = Vec::new();
    for line in input.trim().split('\n') {
        let (position, velocity) = line.trim().split_once(" @ ").unwrap();
        let position = position
            .split(", ")
            .map(|p| p.parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        let velocity = velocity
            .split(", ")
            .map(|v| v.trim().parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        hailstones.push(Hailstone::new(
            (position[0], position[1], position[2]),
            (velocity[0], velocity[1], velocity[2]),
        ));
    }

    hailstones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("input1.txt");
        let hailstones = parse_input(input);
        let valid_intersection_count = count_intersections(&hailstones, 7.0..=27.0);

        assert_eq!(valid_intersection_count, 2);
    }
}
