use crate::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn solve_part_one(&self, input: &str) -> usize {
        return collect_callories(input)
            .into_iter()
            .max()
            .unwrap();
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let mut summed: Vec<usize> = collect_callories(input);
        summed.sort_by(|a, b| b.cmp(a));
        
        return summed.iter().take(3).sum::<usize>();
    }

    fn index(&self) -> usize {
        return 1;
    }
}

fn collect_callories(input: &str) -> Vec<usize> {
    return input.split("\r\n\r\n")
        .map(|line| line.split("\r\n").flat_map(|n| n.parse::<usize>()).sum())
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_sample_1_input() {
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        let day = Day1{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 24000);
    }

    #[test]
    fn part_one_sample_2_input() {
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000\r\n\r\n50000";
        let day = Day1{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 50000);
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(1);
        let day = Day1{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 74198);
    }

    #[test]
    fn part_two_sample_input() {
        let input = "1000\r\n2000\r\n3000\r\n\r\n4000\r\n\r\n5000\r\n6000\r\n\r\n7000\r\n8000\r\n9000\r\n\r\n10000";
        let day = Day1{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 45000);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(1);
        let day = Day1{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 209914);
    }
}
