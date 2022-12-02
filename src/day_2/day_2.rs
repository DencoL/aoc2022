use std::str::FromStr;
use crate::problem::Problem;
use super::play::{Play, ExpectedGameResult};

pub struct Day2 {}

impl Problem for Day2 {
    fn solve_part_one(&self, input: &str) -> usize {
        return play_game(input, |play_identifier, _| Play::from_str(play_identifier).unwrap()) as usize;
    }

    fn solve_part_two(&self, input: &str) -> usize {
        return play_game(input, |expected_game_result_identifier, elf_play| {
            let expected_game_result = ExpectedGameResult::from_str(expected_game_result_identifier);

            match expected_game_result.unwrap() {
                ExpectedGameResult::MyLoss => elf_play.beats(),
                ExpectedGameResult::Draw => elf_play,
                ExpectedGameResult::MyWin => elf_play.loses_to()
            }
        }) as usize;
    }

    fn index(&self) -> usize {
        return 2;
    }

    fn name(&self) -> String {
        return String::from("Rock Paper Scissors");
    }
}

fn play_game(input: &str, get_my_play: fn(&str, Play) -> Play) -> u64 {
    let mut my_total_score: u64 = 0;

    for line in input.lines() {
        let plays: Vec<&str> = line.split(" ").collect();

        let elf_play = Play::from_str(plays[0]).unwrap();
        let my_play = get_my_play(plays[1], elf_play);

        my_total_score += my_play.get_score() as u64;

        if Play::is_draw(&elf_play, &my_play) {
            my_total_score += 3;
        }

        if my_play.can_beat(&elf_play) {
            my_total_score += 6;
        }

    }

    return my_total_score;
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
