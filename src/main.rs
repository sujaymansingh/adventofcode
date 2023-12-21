mod core;
mod maths;
mod string_scanner;
mod y2023;

use structopt::StructOpt;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::core::{CoreError, Day, Part, Solver, Year};

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc", about = "Advent of Code solutions")]
struct Opt {
    #[structopt()]
    year: Year,

    #[structopt()]
    day: Day,

    #[structopt()]
    part: Part,
}

fn main() -> Result<(), CoreError> {
    let opt = Opt::from_args();

    let filename = get_filename(&opt.year, &opt.day);
    let lines = read_lines(&filename)?;

    let mut solver = get_solver(&opt.year, &opt.day, &opt.part);

    for line in lines {
        solver.handle_line(&line?)?;
    }

    let solution = solver.extract_solution()?;
    println!("{}", solution);

    Ok(())
}

fn get_filename(year: &Year, day: &Day) -> PathBuf {
    let short_filename = format!("{}{}.txt", year.to_string(), day.to_string(),);
    PathBuf::from(".").join("inputs").join(short_filename)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_solver(year: &Year, day: &Day, part: &Part) -> Box<dyn Solver> {
    match year.raw_value() {
        2023 => y2023::get_solver(day, part),
        _ => todo!(),
    }
}
