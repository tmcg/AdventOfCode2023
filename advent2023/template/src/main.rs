
pub fn part1() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);

    lines.len().to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);

    lines.len().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "zz");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
