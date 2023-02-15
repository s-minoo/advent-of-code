use std::io::BufRead;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

pub fn solve_day(day: u8, input: impl BufRead) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        4 => day4::solve(input),
        5 => day5::solve(input),
        6 => day6::solve(input),
        7 => day7::solve(input),

        _ => todo!(),
    }
}
