use crate::Solution;

pub mod day1;
pub mod day2;
pub mod day3;

pub fn new(day: &str, input: String) -> Option<Box<dyn Solution>> {
    match day {
        "1" => Some(Box::new(day1::Runner { input })),
        "2" => Some(Box::new(day2::Runner { input })),
        "3" => Some(Box::new(day3::Runner { input })),
        _ => None,
    }
}
