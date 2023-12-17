use itertools::Itertools;

struct History {
    input: Vec<i64>,
}

impl History {
    fn next_row(src: &[i64]) -> Vec<i64> {
        src.iter()
           .tuple_windows::<(_, _)>()
           .map(|x| x.1 - x.0)
           .collect()
    }
    
    fn next_seq(&self) -> i64 {
        let mut v: Vec<Vec<i64>> = vec![self.input.clone()];

        loop {
            let w = v.last().unwrap();
            let x = History::next_row(w);

            if x.iter().all(|a| *a == 0) {
                return v.iter()
                    .map(|a| a.last().unwrap())
                    .sum::<i64>();
            }

            v.push(x);
        }
    }

    fn reverse(&self) -> History {
        let input =
            self.input
                .iter()
                .rev()
                .cloned()
                .collect();

        History {
            input
        }
    }
}

impl From<&str> for History {
    fn from(item: &str) -> Self {
        let input = 
            item.split(' ')
            .map(|x| x.parse::<i64>().expect("Unable to parse number"))
            .collect::<Vec<_>>();

        History {
            input,
        }
    }
}

pub fn part1() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);

    let total =
        lines.iter()
        .map(|x| History::from(x.as_str()))
        .map(|h| h.next_seq())
        .sum::<i64>();

    total.to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);

    let total =
        lines.iter()
        .map(|x| History::from(x.as_str()))
        .map(|h| h.reverse())
        .map(|h| h.next_seq())
        .sum::<i64>();

    total.to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let h = History::from("0 3 6 9 12 15");

        assert_eq!(h.input.len(), 6);
        assert_eq!(h.input, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn solve_next_row() {
        let h = History::from("0 3 6 9 12 15");

        let r1 = History::next_row(&h.input);
        assert_eq!(r1, vec![3, 3, 3, 3, 3]);

        let r2 = History::next_row(&r1);
        assert_eq!(r2, vec![0, 0, 0, 0]);
    }

    #[test]
    fn solve_next_seq() {
        let h1 = History::from("0 3 6 9 12 15");
        assert_eq!(h1.next_seq(), 18);

        let h2 = History::from("1 3 6 10 15 21");
        assert_eq!(h2.next_seq(), 28);

        let h3 = History::from("10 13 16 21 30 45");
        assert_eq!(h3.next_seq(), 68);
    }

    #[test]
    fn solve_sum() {
        let input = include_str!("../input2.txt");
        let lines = shared::input_as_lines(input);
    
        let total =
            lines.iter()
            .map(|x| History::from(x.as_str()))
            .map(|h| h.next_seq())
            .sum::<i64>();
    
        assert_eq!(total, 114);

        let total =
            lines.iter()
            .map(|x| History::from(x.as_str()))
            .map(|h| h.reverse())
            .map(|h| h.next_seq())
            .sum::<i64>();

        assert_eq!(total, 2); 
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "1479011877");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "973");
    }
}
