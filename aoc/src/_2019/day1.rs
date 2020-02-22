pub struct Runner {
	pub input: String,
}

impl crate::Solution for Runner {
	fn run_a(&self) -> String {
		let mut res = 0;
		for l in self.input.lines() {
			let i: u64 = l.trim().parse().expect("Unable to parse input");
			res += fuel(i);
		}
		res.to_string()
	}

	fn run_b(&self) -> String {
		let mut res = 0;
		for l in self.input.lines() {
			let i: u64 = l.trim().parse().expect("Unable to parse input");
			let mut last = fuel(i);
			while last > 0 {
				res += last;
				last = fuel(last);
			}
		}
		res.to_string()
	}
}

fn fuel(mass: u64) -> u64 {
	if mass < 6 {
		return 0;
	}
	(mass / 3) - 2
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Solution;

	#[test]
	fn mass_calc() {
		assert_eq!(fuel(12), 2);
		assert_eq!(fuel(14), 2);
		assert_eq!(fuel(1969), 654);
		assert_eq!(fuel(100756), 33583);
	}

	#[test]
	fn rocket_eq() {
		assert_eq!(
			Runner {
				input: String::from("14")
			}
			.run_b(),
			String::from("2")
		);
		assert_eq!(
			Runner {
				input: String::from("1969")
			}
			.run_b(),
			String::from("966")
		);
		assert_eq!(
			Runner {
				input: String::from("100756")
			}
			.run_b(),
			String::from("50346")
		);
	}
}
