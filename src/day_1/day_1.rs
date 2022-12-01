use crate::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn solve_part_one(&self, input: &str) -> usize {
        let split: Vec<&str> = input.lines().collect();

        let mut values: Vec<Vec<usize>> = vec![vec![]];
        let mut index = 0;
        for s in split.into_iter() {
            if s == "" {
                index += 1;
                values.push(vec![]);
                continue;
            }

            values[index].push(s.parse::<usize>().unwrap());
        }

        return values
            .iter()
            .map(|v| v.iter().sum())
            .max()
            .unwrap();
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let split: Vec<&str> = input.lines().collect();

        let mut values: Vec<Vec<usize>> = vec![vec![]];
        let mut index = 0;
        for s in split.into_iter() {
            if s == "" {
                index += 1;
                values.push(vec![]);
                continue;
            }

            values[index].push(s.parse::<usize>().unwrap());
        }

        let mut summed: Vec<usize> = values
            .iter()
            .map(|v| v.iter().sum())
            .collect();

        summed.sort_by(|a, b| b.cmp(a));

        return summed.iter().take(3).sum::<usize>();
    }

    fn index(&self) -> usize {
        return 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    use super::*;

    #[test]
    fn part_one_sample_1_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let day = Day1{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 24000);
    }

    #[test]
    fn part_one_sample_2_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000

50000";
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
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let day = Day1{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 45000);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(1);
        let day = Day1{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 74198);
    }
}
