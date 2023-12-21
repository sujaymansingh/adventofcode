use crate::core::{Day, Part, Solver};

mod d01;

pub fn get_solver(day: &Day, part: &Part) -> Box<dyn Solver> {
    match (day.raw_value(), part.raw_value()) {
        (1, 1) => d01::part_1(),
        (1, 2) => d01::part_2(),
        _ => todo!(),
    }
}