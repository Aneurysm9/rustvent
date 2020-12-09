use std::fs;
use std::time::Instant;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rustvent", about = "Advent of Rust.")]
struct Opt {
    // Year to run
    #[structopt(short, long, default_value = "2020")]
    year: String,

    // Day to run
    #[structopt(short, long)]
    day: String,

    // Part to run
    #[structopt(short, long, default_value = "both")]
    part: String,

    // Input filename
    #[structopt(short, long)]
    input: Option<String>,

    // Print runtime
    #[structopt(short, long)]
    timing: bool,
}

fn main() {
    let opt = Opt::from_args();
    let input;

    match opt.input {
        Some(i) => input = i,
        None => input = format!("input/{}/day{}.in", opt.year, opt.day),
    }

    let start = Instant::now();

    match aoc::new(
        &opt.year,
        &opt.day,
        fs::read_to_string(&input).expect("Error reading file"),
    ) {
        Some(d) => match opt.part.as_str() {
            "a" => println!("{}", d.run_a()),
            "b" => println!("{}", d.run_b()),
            "both" => {
                println!("{}", d.run_a());
                println!("{}", d.run_b());
            }
            _ => panic!("puzzle part must be 'a', 'b', or 'both'."),
        },
        None => panic!("Unknown puzzle requested."),
    }

    if opt.timing {
        println!("{:?}", start.elapsed());
    }
}
