#![allow(dead_code)]
mod advent01;
/* mod scratch; */
mod shared;


fn main() {
    use advent01::{part1, part2};
    println!("{}", part1());
    println!("{}", part2());
}