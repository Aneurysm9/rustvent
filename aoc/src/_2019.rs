use crate::Solution;

pub mod day1;

pub fn new(day: &str, input: String) -> Option<impl Solution> {
	match day {
		"1" => Some(day1::Runner { input: input }),
		_ => None,
	}
}
