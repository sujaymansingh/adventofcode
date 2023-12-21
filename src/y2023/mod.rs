use crate::core::{Day, Part, Solver};

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;

pub fn get_solver(day: &Day, part: &Part) -> Box<dyn Solver> {
    match (day.raw_value(), part.raw_value()) {
        (1, 1) => d01::part_1(),
        (1, 2) => d01::part_2(),
        (2, 1) => d02::part_1(),
        (2, 2) => d02::part_2(),
        (3, 1) => d03::part_1(),
        (3, 2) => d03::part_2(),
        (4, 1) => d04::part_1(),
        (4, 2) => d04::part_2(),
        (5, 1) => d05::part_1(),
        (5, 2) => d05::part_2(),
        (6, 1) => d06::part_1(),
        (6, 2) => d06::part_2(),
        (7, 1) => d07::part_1(),
        (7, 2) => d07::part_2(),
        (8, 1) => d08::part_1(),
        (8, 2) => d08::part_2(),
        _ => todo!(),
    }
}
