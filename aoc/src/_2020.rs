use crate::Solution;

pub mod day1;
pub mod day2;

pub fn new(day: &str, input: String) -> Option<Box<dyn Solution>> {
    match day {
        "1" => Some(Box::new(day1::Runner { input })),
        "2" => Some(Box::new(day2::Runner { input })),
        _ => None,
    }
}
