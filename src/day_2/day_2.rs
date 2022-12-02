use std::{fmt, str::FromStr};
use crate::problem::Problem;

pub struct Day2 {}

#[derive(PartialEq)]
pub enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3
}

impl fmt::Display for Play {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Play::Rock => write!(f, "A"),
            Play::Paper => write!(f, "B"),
            Play::Scissors => write!(f, "C")
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(())
        }
    }
}

impl Problem for Day2 {
    fn solve_part_one(&self, input: &str) -> usize {
        let lines = input.lines();

        let mut my_total_score: u64 = 0;
        for line in lines {
            let plays: Vec<&str> = line.split(" ").collect();

            let elf_play = Play::from_str(plays[0]).unwrap();
            let my_play = Play::from_str(translate_my_play(plays[1])).unwrap();

            my_total_score += my_play as u64;

            if is_draw(&elf_play, &my_play) {
                my_total_score += 3;
            }

            if is_my_win(&elf_play, &my_play) {
                my_total_score += 6;
            }

        }

        return my_total_score as usize;
    }

    fn solve_part_two(&self, input: &str) -> usize {
        let lines = input.lines();

        let mut my_total_score: u64 = 0;
        for line in lines {
            let plays: Vec<&str> = line.split(" ").collect();

            let elf_play = Play::from_str(plays[0]).unwrap();
            let my_play = get_expected_play(plays[1], &elf_play);

            my_total_score += my_play as u64;

            if is_draw(&elf_play, &my_play) {
                my_total_score += 3;
            }

            if is_my_win(&elf_play, &my_play) {
                my_total_score += 6;
            }

        }

        return my_total_score as usize;
    }

    fn index(&self) -> usize {
        return 2;
    }
}

fn is_draw(elf_play: &Play, my_play: &Play) -> bool {
    return elf_play == my_play;
}

fn is_my_win(elf_play: &Play, my_play: &Play) -> bool {
    return match my_play {
        Play::Rock => *elf_play == Play::Scissors,
        Play::Paper => *elf_play == Play::Rock,
        Play::Scissors => *elf_play == Play::Paper,
    }
}

fn get_expected_play(expected_result: &str, elf_play: &Play) -> Play  {
    let loss = "X";
    let draw = "Y";
    let win = "Z";

    if expected_result == loss {
        return match elf_play {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper
        }
    }

    if expected_result == draw {
        return match elf_play {
            Play::Rock => Play::Rock,
            Play::Paper => Play::Paper,
            Play::Scissors => Play::Scissors
        }
    }

    if expected_result == win {
        return match elf_play {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock
        }
    }

    panic!();
}

fn translate_my_play(my_play: &str) -> &str {
    return match my_play {
        "X" => "A",
        "Y" => "B",
        "Z" => "C",
        _ => panic!("invalid play {}", my_play)
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
    fn part_one_sample_input() {
        let input = "A Y
B X
C Z";
        let day = Day2{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 15);
    }

    #[test]
    fn part_one_my_input() {
        let input = read_input(2);
        let day = Day2{};

        let result = day.solve_part_one(&input);

        assert_eq!(result, 14375);
    }

    #[test]
    fn part_two_sample_input() {
        let input = "A Y
B X
C Z";
        let day = Day2{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 12);
    }

    #[test]
    fn part_two_my_input() {
        let input = read_input(2);
        let day = Day2{};

        let result = day.solve_part_two(&input);

        assert_eq!(result, 10274);
    }
}
