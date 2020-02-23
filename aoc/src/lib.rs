pub mod _2019;

pub trait Solution {
    fn run_a(&self) -> String;
    fn run_b(&self) -> String;
}

pub fn new(year: &str, day: &str, input: String) -> Option<impl Solution> {
    match year {
        "2019" => _2019::new(day, input),
        _ => None,
    }
}
