use std::collections::{VecDeque, HashSet};

use crate::problem::Problem;

pub struct Day12 {}

type Vertex = (usize, usize);

impl Problem for Day12 {
    fn solve_part_one(&self, input: &str) -> usize {
        let (mut grid, start_point, destination_point) = build_grid(input);
        grid[start_point.0][start_point.1] = b'a';
        grid[destination_point.0][destination_point.1] = b'z';

        if let Some(distance) = bfs(&grid, start_point, destination_point) {
            return distance;
        }

        return 0;
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let (mut grid, _, destination_point) = build_grid(input);
        grid[destination_point.0][destination_point.1] = b'z';

        let mut all_lowest_start_points: Vec<Vertex> = vec![];
        for (row_index, row) in grid.iter().enumerate() {
            for (node_index, node) in row.iter().enumerate() {
                if *node == b'a' {
                    all_lowest_start_points.push((row_index, node_index));
                }
            }
        }

        let mut distances_from_all_lowest_points: Vec<usize> = vec![];
        for lowest_start_point in all_lowest_start_points {
            if let Some(distance) = bfs(&grid, lowest_start_point, destination_point) {
                distances_from_all_lowest_points.push(distance);
            }
        }

        return distances_from_all_lowest_points.into_iter().min().unwrap();
    }

    fn index(&self) -> usize {
        return 12;
    }

    fn name(&self) -> String {
        return String::from("Hill Climbing Algorithm");
    }
}

fn build_grid(input: &str) -> (Vec<Vec<u8>>, Vertex, Vertex) {
    let mut start_position: Option<Vertex> = None;
    let mut destination_position: Option<Vertex> = None;

    let grid = input
        .lines()
        .into_iter()
        .enumerate()
        .map(|(row, line)| {
            return line
                .as_bytes()
                .iter()
                .enumerate()
                .map(|(column, node)| {
                    if start_position == None {
                        if *node == b'S' {
                            start_position = Some((row, column))
                        }
                    }

                    if destination_position == None {
                        if *node == b'E' {
                            destination_position = Some((row, column))
                        }
                    }

                    return *node as u8;
                })
                .collect();
        })
        .collect();

    return (grid, start_position.unwrap(), destination_position.unwrap());
}

fn bfs(grid: &Vec<Vec<u8>>, start_point: Vertex, end_point: Vertex) -> Option<usize> {
    let mut queue = VecDeque::from([(start_point, 0)]);
    let mut visited: HashSet<Vertex> = HashSet::from([start_point]);

    while let Some((vertex, distance_from_start)) = queue.pop_back() {
        if vertex == end_point {
            return Some(distance_from_start);
        }

        for neighbor_vertex in get_neighbors(grid, vertex) {
            if !visited.contains(&neighbor_vertex) {
                visited.insert(neighbor_vertex);
                queue.push_front((neighbor_vertex, distance_from_start + 1));
            }
        }
    }

    return None;
}

fn get_neighbors(grid: &Vec<Vec<u8>>, vertex: Vertex) -> Vec<Vertex> {
    let mut neighbors = vec![];

    for (x_boundary, y_boundary) in vec![(-1, 0), (0, 1), (1, 0), (0, -1)] {
        let (neighbor_row, neighbor_column) = ((x_boundary + vertex.0 as i32) as usize, (y_boundary + vertex.1 as i32) as usize);

        if let Some(neighbor) = grid.get(neighbor_row).and_then(|row| row.get(neighbor_column)) {
            if grid[vertex.0][vertex.1] + 1 >= *neighbor {
                neighbors.push((neighbor_row, neighbor_column));
            }
        }
    }

    return neighbors;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn part_one_sample_input() {
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

        let day = Day12 {};

        let result = day.solve_part_one(input);

        assert_eq!(result, 31);
    }

    // #[test]
    fn part_one_my_input() {
        let input = read_input(11);
        let day = Day12 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 110888);
    }

    // #[test]
    fn part_two_sample_input() {
        let day = Day12 {};

        let result =
            day.solve_part_two(&std::fs::read_to_string("src/day_11/day_11_sample.txt").unwrap());

        assert_eq!(result, 2713310158);
    }

    // #[test]
    fn part_two_my_input() {
        let input = read_input(11);
        let day = Day12 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 25590400731);
    }
}
