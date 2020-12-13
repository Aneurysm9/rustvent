use crate::Solution;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub fn new(day: &str, input: String) -> Option<Box<dyn Solution>> {
    match day {
        "1" => Some(Box::new(day1::Runner { input })),
        "2" => Some(Box::new(day2::Runner { input })),
        "3" => Some(Box::new(day3::Runner { input })),
        "4" => Some(Box::new(day4::Runner { input })),
        "5" => Some(Box::new(day5::Runner { input })),
        "6" => Some(Box::new(day6::Runner { input })),
        "7" => Some(Box::new(day7::Runner { input })),
        "8" => Some(Box::new(day8::Runner { input })),
        "9" => Some(Box::new(day9::Runner { input })),
        "10" => Some(Box::new(day10::Runner { input })),
        "11" => Some(Box::new(day11::Runner { input })),
        "12" => Some(Box::new(day12::Runner { input })),
        _ => None,
    }
}
