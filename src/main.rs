use aoc;
use aoc::Solution;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(d) = aoc::new(
        &args[1],
        &args[2],
        fs::read_to_string(&args[3]).expect("Error reading file"),
    ) {
        println!("{}", d.run_a());
        println!("{}", d.run_b());
    } else {
        panic!("Unknown puzzle requested.");
    }
}
