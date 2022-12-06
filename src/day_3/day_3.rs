use crate::problem::Problem;
use std::collections::HashSet;

pub struct Day3 {}

impl Problem for Day3 {
    fn solve_part_one(&self, input: &str) -> String {
        let mut common_items: Vec<char> = vec![];

        for rucksack in input.lines() {
            let half_size = rucksack.len() / 2;

            let (first_half, second_half) = rucksack.split_at(half_size);
            let first_half: HashSet<char> = first_half.chars().collect();
            let second_half: HashSet<char> = second_half.chars().collect();
           
            for item in first_half {
                if second_half.contains(&item) {
                    common_items.push(item);
                }
            }
        }

        return get_items_priority_sum(common_items).to_string();
    }

    fn solve_part_two(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        let mut groups: Vec<Vec<&str>> = vec![];

        let mut index = 0;
        loop {
            if index == lines.len() - 1 {
                groups.push(lines[..index + 2].to_vec());
            }
        
            if index + 2 == lines.len() - 1 {
                groups.push(lines[index..].to_vec());
                break;
            }
        
            groups.push(lines[index..index + 3].to_vec());
        
            index += 3;
        }

        let mut common_items: Vec<char> = vec![];
        for group in groups {
            let first_group_items: Vec<char> = group[0].chars().collect();

            let mut already_added: Vec<char> = vec![];
            for item in first_group_items {
                if already_added.contains(&item) {
                    continue;
                }

                if group[1].contains(item) && group[2].contains(item) {
                    already_added.push(item);
                    common_items.push(item);
                }
            }
        }

        return get_items_priority_sum(common_items).to_string();
    }

    fn index(&self) -> usize {
        return 3;
    }

    fn name(&self) -> String {
        return String::from("Rucksack Reorganization");
    }
}

fn get_items_priority_sum(items: Vec<char>) -> usize {
    const LOWER_PRIORITY_BOUNDARY: u8 = b'a' - 1;
    const UPPER_PRIORITY_BOUNDARY: u8 = b'A' - 27;

    return items.iter().map(|item| {
        let item_ascii = *item as u8;

        let mut subtract_boundary = UPPER_PRIORITY_BOUNDARY;
        if item.is_ascii_lowercase() {
            subtract_boundary = LOWER_PRIORITY_BOUNDARY;
        }

        return (item_ascii - subtract_boundary) as usize;
    }).sum();
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_sample_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let day = Day3{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "157");
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(3);
        let day = Day3{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "8153");
    }

    #[test]
    fn part_two_sample_input() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let day = Day3{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "70");
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(3);
        let day = Day3{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "2342");
    }
}
