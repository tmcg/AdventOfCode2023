#![allow(dead_code)]
mod advent02;
/* mod scratch; */
mod shared;

fn main() {
    use advent02::{part1, part2};
    println!("{}", part1());
    println!("{}", part2());
}