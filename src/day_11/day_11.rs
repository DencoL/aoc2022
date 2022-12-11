use std::cell::RefCell;

use crate::problem::Problem;
use super::monkey::Monkey;

pub struct Day11 {}

impl Problem for Day11 {
    fn solve_part_one(&self, input: &str) -> usize {
        return solve(&collect_monkeys(input), 20, |i| i / 3);
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let monkeys = collect_monkeys(input);
        let modulos: u64 = monkeys.iter().map(|m| m.test_number).product();
        return solve(&monkeys, 10_000, |i| i % modulos);
    }

    fn index(&self) -> usize {
        return 11;
    }

    fn name(&self) -> String {
        return String::from("Monkey in the middle");
    }
}

fn solve(monkeys: &Vec<Monkey>, rounds: u32, worry_fn: impl Fn(u64) -> u64) -> usize {
    let mut inspections: Vec<u64> = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for (monkey_index, monkey) in monkeys.iter().enumerate() {
            for item in monkey.inspect_items(&worry_fn) {
                let receiving_monkey_index = monkey.test_item(item);
                let receiving_monkey: &Monkey = &monkeys[receiving_monkey_index as usize];
                monkey.throw_at_monkey(item, &receiving_monkey);
                inspections[monkey_index] += 1;
            }
        }
    }

    inspections.sort_by(|a, b| b.cmp(a));
    return inspections.iter().take(2).product::<u64>() as usize;
}

fn collect_monkeys(input: &str) -> Vec<Monkey> {
    return input.split("\r\n\r\n").map(|monkey| {
        let lines: Vec<&str> = monkey.lines().collect();
        let items: Vec<u64> = lines[1]
            .replace(",", " ")
            .split(" ")
            .map(|v| v.parse().unwrap_or(0))
            .filter(|v| *v != 0)
            .collect();

        let operation_split: Vec<&str> = lines[2].split(" ").collect();

        return Monkey {
            items: RefCell::new(items),
            operation: operation_split[operation_split.len() - 2].chars().collect::<Vec<char>>()[0],
            operation_input: operation_split.last().unwrap().parse().unwrap_or(0),
            test_number: lines[3].split(" ").last().unwrap().parse().unwrap(),
            test_true_monkey: lines[4].split(" ").last().unwrap().parse().unwrap(),
            test_false_monkey: lines[5].split(" ").last().unwrap().parse().unwrap()
        };
    }).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;

    #[test]
    fn part_one_sample_input() {
        let day = Day11 {};

        let result = day.solve_part_one(&std::fs::read_to_string("src/day_11/day_11_sample.txt").unwrap());

        assert_eq!(result, 10605);
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(11);
        let day = Day11 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 110888);
    }

    #[test]
    fn part_two_sample_input() {
        let day = Day11 {};

        let result = day.solve_part_two(&std::fs::read_to_string("src/day_11/day_11_sample.txt").unwrap());

        assert_eq!(result, 2713310158);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(11);
        let day = Day11 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 25590400731);
    }
}
