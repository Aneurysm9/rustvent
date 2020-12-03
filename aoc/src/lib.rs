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
