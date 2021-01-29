use crate::Solution;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod intcode;

pub fn new(day: &str, input: String) -> Option<Box<dyn Solution>> {
    match day {
        "1" => Some(Box::new(day1::Runner { input })),
        "2" => Some(Box::new(day2::Runner { input })),
        "3" => Some(Box::new(day3::Runner { input })),
        "4" => Some(Box::new(day4::Runner { input })),
        "5" => Some(Box::new(day5::Runner { input })),
        "6" => Some(Box::new(day6::Runner { input })),
        _ => None,
    }
}
