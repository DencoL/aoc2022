use std::{env, fs};
use crate::problem::Problem;
use day_1::day_1::Day1;

mod problem;
mod day_1;
mod utils;

fn main() {
    let days: Vec<Box<dyn Problem>> = vec![
        Box::new(Day1 {}),
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        days.iter().for_each(|day| solve_day(day));
    } else {
        let day_index: usize = args[1].parse().unwrap();
        let day = &days
            .into_iter()
            .filter(|day| day.index() == day_index)
            .collect::<Vec<Box<dyn Problem>>>()[0];

        solve_day(&day);
    }
}

fn solve_day(day: &Box<dyn Problem>) {
    let input = read_input(day.index());

    print!("Day {}\t", day.index());
    print!("\t{}\t", day.solve_part_one(&input));
    println!("\t{}\t", day.solve_part_two(&input));
}

fn read_input(day_number: usize) -> String {
    let day_string = format!("day_{}", day_number);

    return fs::read_to_string(format!("src/{}/{}.txt", day_string, day_string)).unwrap();
}
