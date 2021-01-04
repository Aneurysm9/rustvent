use std::fs;

#[macro_use]
extern crate lazy_static;

pub mod _2019;
pub mod _2020;

pub trait Solution {
    fn run_a(&self) -> String;
    fn run_b(&self) -> String;
}

pub fn new(year: &str, day: &str, input: String) -> Option<Box<dyn Solution>> {
    match year {
        "2019" => _2019::new(day, input),
        "2020" => _2020::new(day, input),
        _ => None,
    }
}

pub fn read_input(year: usize, suffix: &str) -> String {
    fs::read_to_string(format!("../input/{}/day{}.in", year, suffix)).expect("Error reading file")
}
