use crate::problem::Problem;

pub struct Day3 {}

impl Problem for Day3 {
    fn solve_part_one(&self, input: &str) -> usize {
        let mut common_items: Vec<char> = vec![];

        for rucksack in input.lines() {
            let items: Vec<char> = rucksack.split("").flat_map(|i| i.chars()).collect();
            let half_size = items.len() / 2;

            let first_half: &[char] = &items[..half_size];
            let second_half: &[char] = &items[half_size..];

            let mut already_added: Vec<&char> = vec![];
            for item in first_half {
                if already_added.contains(&item) {
                    continue;
                }

                if second_half.contains(item) {
                    common_items.push(*item);
                    already_added.push(&item);
                }
            }
        }

        return get_items_priority_sum(common_items);
    }

    fn solve_part_two(&self, input: &str) -> usize {
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

        return get_items_priority_sum(common_items);
    }

    fn index(&self) -> usize {
        return 3;
    }

    fn name(&self) -> String {
        return String::from("Rucksack Reorganization");
    }
}

fn get_items_priority_sum(items: Vec<char>) -> usize {
    return items.iter().map(|item| {
        let item_ascii = *item as usize;

        let mut priority = 0;
        if item_ascii >= 97 && item_ascii <= 122 {
            priority = item_ascii - 96; 
        }

        if item_ascii >= 65 && item_ascii <= 90 {
            priority = item_ascii - 38;
        }

        return priority;
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

        assert_eq!(result, 157);
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(3);
        let day = Day3{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 8153);
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

        assert_eq!(result, 70);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(3);
        let day = Day3{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 2342);
    }
}
