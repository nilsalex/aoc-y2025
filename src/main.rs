#![feature(test)]

use std::env;

mod day01;
mod day02;
mod day03;

pub fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.as_str() {
            "01" => day01::main(),
            "02" => day02::main(),
            "03" => day03::main(),
            _ => {
                panic!("invalid argument for \"day\": {}", day)
            }
        }
    } else {
        day01::main();
        day02::main();
        day03::main();
    }
}
