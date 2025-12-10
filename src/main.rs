#![feature(test)]

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub fn main() {
    if let Some(day) = env::args().nth(1) {
        match day.as_str() {
            "01" => day01::main(),
            "02" => day02::main(),
            "03" => day03::main(),
            "04" => day04::main(),
            "05" => day05::main(),
            "06" => day06::main(),
            "07" => day07::main(),
            "08" => day08::main(),
            _ => {
                panic!("invalid argument for \"day\": {}", day)
            }
        }
    } else {
        day01::main();
        day02::main();
        day03::main();
        day04::main();
        day05::main();
        day06::main();
        day07::main();
        day08::main();
    }
}
