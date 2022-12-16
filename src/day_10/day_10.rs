use crate::problem::Problem;

pub struct Day10 {}

impl Problem for Day10 {
    fn solve_part_one(&self, input: &str) -> usize {
        let mut cycle = 1;
        let mut x = 1;
        let mut signal_strength = 0;
        let mut next_cycle = 20;

        for line in input.lines() {
            let split = line.split(" ").collect::<Vec<&str>>();

            if split.len() == 1 {
                cycle += 1;

                if cycle == next_cycle {
                    signal_strength += cycle * x;
                    next_cycle += 40;
                }
            } else {
                for i in 0..2 {
                    cycle += 1;

                    if i == 1 {
                        let value_change: i32 = split[1].parse().unwrap();
                        x += value_change;
                    }

                    if cycle == next_cycle {
                        signal_strength += cycle * x;
                        next_cycle += 40;
                    }
                }
            }
        }

        return signal_strength as usize;
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let mut cycle = 1;
        let mut x = 1;
        let mut sprite_position = 1;
        let mut column = 0;

        let mut result = String::from("");
        for line in input.lines() {
            let split = line.split(" ").collect::<Vec<&str>>();

            // println!("start cycle {}: begin executing {} ", cycle, line);
            if split.len() == 1 {
                // println!("during cycle {}: crt draws pixel in position {} ", cycle, cycle - 1);
                if column >= sprite_position - 1 && column <= sprite_position + 1 {
                    result += &"#";
                    print!("#");
                } else {
                    result += &".";
                    print!(".")
                }
                // println!("current crt raw {}", result);
                cycle += 1;
                column += 1;
            } else {
                for i in 0..2 {
                    // println!("during cycle {}: crt draws pixel in position {} ", cycle, column);
                    if column >= sprite_position - 1 && column <= sprite_position + 1 {
                        result += &"#";
                        print!("#");
                    } else {
                        result += &".";
                        print!(".");
                    }
                    // println!("current crt raw {}", result);

                    if i == 1 {
                        let value_change: i32 = split[1].parse().unwrap();
                        x += value_change;
                        sprite_position = x;
                        // println!("end of cycle {} finishing executing {} (register x is not {})",cycle, line, x);
                        // println!("sprite position {}", sprite_position);
                    }

                    cycle += 1;
                    column += 1;
                    // println!();
                    if column % 40 == 0 {
                        column = 0;
                        println!();
                    }
                }
            }
        }

        return 0;
    }

    fn index(&self) -> usize {
        return 10;
    }

    fn name(&self) -> String {
        return String::from("Cathode-ray Tube");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_input;
    use test_case::test_case;

    // #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    // #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    // #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    // #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    // #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn part_one_sample_input(input: &str, expected_result: usize) {
        let day = Day10 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, expected_result);
    }

    // #[test]
    fn part_one_my_input() {
        let input = read_input(6);
        let day = Day10 {};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 1723);
    }

    // #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    // #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    // #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    // #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    // #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn part_two_sample_input(input: &str, expected_result: usize) {
        let day = Day10 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, expected_result);
    }

    // #[test]
    fn part_two_my_input() {
        let input = read_input(6);
        let day = Day10 {};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 3708);
    }
}
