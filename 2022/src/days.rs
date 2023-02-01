use std::io::BufRead;


mod day1;
mod day2;


pub fn solve_day(day: u8, input: impl BufRead) {
    match day {
        1 => day1::solve(input),
        2 => day2::solve(input), 
        _ => todo!(),
    }
}
