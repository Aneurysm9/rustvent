use crate::Solution;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day22;
pub mod day23;
pub mod day24;
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
        "13" => Some(Box::new(day13::Runner { input })),
        "14" => Some(Box::new(day14::Runner { input })),
        "15" => Some(Box::new(day15::Runner { input })),
        "16" => Some(Box::new(day16::Runner { input })),
        "17" => Some(Box::new(day17::Runner { input })),
        "18" => Some(Box::new(day18::Runner { input })),
        "19" => Some(Box::new(day19::Runner { input })),
        "22" => Some(Box::new(day22::Runner { input })),
        "23" => Some(Box::new(day23::Runner { input })),
        "24" => Some(Box::new(day24::Runner { input })),
        _ => None,
    }
}
