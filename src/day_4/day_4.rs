use crate::problem::Problem;
use super::pair::Pair;

pub struct Day4 {}

impl Problem for Day4 {
    fn solve_part_one(&self, input: &str) -> String {
        return count_pairs(input, |first_pair, second_pair| {
            first_pair.is_inside_pair(&second_pair) || second_pair.is_inside_pair(&first_pair)
        }).to_string();
    }

    fn solve_part_two(&self, input: &str) -> String {
        return count_pairs(input, |first_pair, second_pair| {
            first_pair.overlaps_with_pair(&second_pair) || second_pair.overlaps_with_pair(&first_pair)
        }).to_string();
    }

    fn index(&self) -> usize {
        return 4;
    }

    fn name(&self) -> String {
        return String::from("Camp cleanup");
    }
}

fn count_pairs(input: &str, compare_pairs: fn(Pair, Pair) -> bool) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let (first_pair, second_pair) = parse_pairs(line);

        if compare_pairs(first_pair, second_pair) {
            result += 1;
        }
    }

    return result;
}

fn parse_pairs(line_of_pairs: &str) -> (Pair, Pair) {
    let pairs: Vec<&str> = line_of_pairs.split(",").collect();

    return (Pair::new(pairs[0]), Pair::new(pairs[1]));
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_sample_input() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let day = Day4 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "2");
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(4);
        let day = Day4 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, "485");
    }

    #[test]
    fn part_two_sample_input() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let day = Day4 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "4");
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(4);
        let day = Day4 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, "857");
    }
}
