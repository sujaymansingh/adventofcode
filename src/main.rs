mod core;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code solutions")]
struct Opt {
    #[structopt()]
    year: core::Year,

    #[structopt()]
    day: core::Day,

    #[structopt()]
    part: core::Part,
}

fn main() {
    let opt = Opt::from_args();
    println!(
        "{} {} {}",
        opt.year.to_string(),
        opt.day.to_string(),
        opt.part.to_string()
    );
}
