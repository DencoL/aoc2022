use std::collections::HashSet;

use crate::problem::Problem;
pub struct Day6 {}

impl Problem for Day6 {
    fn solve_part_one(&self, input: &str) -> usize {
        return find_marker_start(&input, 4);
    }

    fn solve_part_two(&self, input: &str) -> usize {
        return find_marker_start(&input, 14)
    }

    fn index(&self) -> usize {
        return 6;
    }

    fn name(&self) -> String {
        return String::from("Tunning trouble");
    }
}

fn find_marker_start(input: &str, boundary: usize) -> usize {
    let input = input.trim();

    for (index, _) in input.chars().into_iter().enumerate() {
        let fours: HashSet<char> = input[index..index + boundary].chars().collect();

        if fours.len() == boundary {
            return index + boundary;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    use test_case::test_case;
    use super::*;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn part_one_sample_input(input: &str, expected_result: usize) {
        let day = Day6{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(6);
        let day = Day6{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 1723);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn part_two_sample_input(input: &str, expected_result: usize) {
        let day = Day6{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(6);
        let day = Day6{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 3708);
    }
}
