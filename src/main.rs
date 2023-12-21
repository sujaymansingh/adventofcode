mod core;

use structopt::StructOpt;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::core::{Day, Part, Year};

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

fn main() -> Result<(), io::Error> {
    let opt = Opt::from_args();

    let filename = get_filename(&opt.year, &opt.day);
    let lines = read_lines(&filename)?;

    for line in lines {
        println!("{}, ", line?);
    }

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
