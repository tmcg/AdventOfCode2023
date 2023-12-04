#![allow(dead_code)]
mod advent03;
/* mod scratch; */
mod shared;

fn main() {
    use advent03::{part1, part2};
    println!("{}", part1());
    println!("{}", part2());
}