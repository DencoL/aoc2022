use std::vec;

use crate::problem::Problem;
pub struct Day5 {}

impl Problem for Day5 {
    fn solve_part_one(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();

        let mut stacks_count = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("move") {
                let stack_numbers: Vec<u32> = lines[i - 2]
                    .split(" ")
                    .filter(|v| *v != "")
                    .map(|v| v.parse().unwrap())
                    .collect();
                
                stacks_count = *stack_numbers.iter().max().unwrap(); break;

            }
        }

        let mut stacks: Vec<Vec<String>> = vec![];

        for _ in 0..stacks_count {
            stacks.push(vec![]);
        }

        let mut movement_index = 0;
        loop {
            let line = lines[movement_index];
            if !line.contains("[") {
                break;
            }

            let items: Vec<String> = line
                .replace("[", "")
                .replace("]", "")
                .replace("    ", " ")
                .split(" ")
                .map(|v| String::from(v))
                .collect();

            for (i, item) in items.into_iter().enumerate() {
                if item == "" {
                    continue;
                }

                stacks[i].insert(0, item);
            }

            movement_index += 1;
        }

        movement_index += 2;

        while movement_index != lines.len() {
            let movement: Vec<usize> = lines[movement_index]
                .split(" ")
                .map(|v| {
                    if let Ok(result) = v.parse::<usize>() {
                        return result;
                    }

                    return 0;
                })
                .filter(|v| *v != 0)
                .collect();
        
            let how_much = movement[0] as usize;
            let from = movement[1] as usize;
            let to = movement[2] as usize;
            for _ in 0..how_much {
                let from_stack = &mut stacks[from - 1];

                if from_stack.len() <= 0 {
                    continue;
                }

                let removed_item = from_stack.remove(from_stack.len() - 1);
                let _ = &stacks[to - 1].push(removed_item);
        
            }
        
            movement_index += 1;
        }

        return stacks.iter().map(|v| v.last().unwrap().clone()).collect();
    }

    fn solve_part_two(&self, input: &str) -> String {
        let lines: Vec<&str> = input.lines().collect();

        let mut stacks_count = 0;
        for (i, line) in lines.iter().enumerate() {
            if line.contains("move") {
                let stack_numbers: Vec<u32> = lines[i - 2]
                    .split(" ")
                    .filter(|v| *v != "")
                    .map(|v| v.parse().unwrap())
                    .collect();
                
                stacks_count = *stack_numbers.iter().max().unwrap(); break;

            }
        }

        let mut stacks: Vec<Vec<String>> = vec![];

        for _ in 0..stacks_count {
            stacks.push(vec![]);
        }

        let mut movement_index = 0;
        loop {
            let line = lines[movement_index];
            if !line.contains("[") {
                break;
            }

            let items: Vec<String> = line
                .replace("[", "")
                .replace("]", "")
                .replace("    ", " ")
                .split(" ")
                .map(|v| String::from(v))
                .collect();

            for (i, item) in items.into_iter().enumerate() {
                if item == "" {
                    continue;
                }

                stacks[i].insert(0, item);
            }

            movement_index += 1;
        }

        movement_index += 2;

        while movement_index != lines.len() {
            let movement: Vec<usize> = lines[movement_index]
                .split(" ")
                .map(|v| {
                    if let Ok(result) = v.parse::<usize>() {
                        return result;
                    }

                    return 0;
                })
                .filter(|v| *v != 0)
                .collect();
        
            let how_much = movement[0] as usize;
            let from = movement[1] as usize;
            let to = movement[2] as usize;

            let mut removed_items: Vec<String> = vec![];
            for _ in 0..how_much {
                let from_stack = &mut stacks[from - 1];

                if from_stack.len() <= 0 {
                    continue;
                }

                removed_items.push(from_stack.remove(from_stack.len() - 1));
        
            }

            removed_items.reverse();
            for removed_item in removed_items {
                let _ = &stacks[to - 1].push(removed_item);
            }
        
            movement_index += 1;
        }

        return stacks.iter().map(|v| v.last().unwrap().clone()).collect();
    }

    fn index(&self) -> usize {
        return 5;
    }

    fn name(&self) -> String {
        return String::from("Supply Stacks");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_sample_input() {
        let input = fs::read_to_string("src/day_5/day_5_sample.txt").unwrap();
        let day = Day5{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(5);
        let day = Day5{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "SHQWSRBDL");
    }

    #[test]
    fn part_two_sample_input() {
        let input = fs::read_to_string("src/day_5/day_5_sample.txt").unwrap();
        let day = Day5{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "MCD");
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(5);
        let day = Day5{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "CDTQZHBRS");
    }
}
