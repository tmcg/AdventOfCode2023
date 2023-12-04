#![allow(dead_code)]
mod advent04;
/* mod scratch; */
mod shared;

fn main() {
    use advent04::{part1, part2};
    println!("{}", part1());
    println!("{}", part2());
}