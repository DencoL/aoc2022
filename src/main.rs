use std::{env, fs};
use crate::problem::Problem;
use day_1::day_1::Day1;
use day_2::day_2::Day2;
use day_3::day_3::Day3;

mod problem;
mod utils;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let days: Vec<Box<dyn Problem>> = vec![
        Box::new(Day1 {}),
        Box::new(Day2 {}),
        Box::new(Day3 {}),
    ];
    
    let args: Vec<String> = env::args().collect();
    
    println!(
        "{0: <3} | {1: <25} | {2: <20} | {3: <20}",
        "", "Day", "Part 1", "Part 2"
    );

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
    let input: String = read_input(day.index());

    println!("{0: <3} | {1: <25} | {2: <20} | {3: <20}",
             day.index(),
             day.name(),
             day.solve_part_one(&input),
             day.solve_part_two(&input));
}

fn read_input(day_number: usize) -> String {
    let day_string = format!("day_{}", day_number);

    return fs::read_to_string(format!("src/{}/{}.txt", day_string, day_string)).unwrap();
}
