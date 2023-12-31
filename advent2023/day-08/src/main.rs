use std::collections::HashMap;
use regex::Regex;

#[allow(dead_code)]
struct CamelNetwork {
    inst: String,
    nodes: HashMap<String, (String, String)>,
}

#[allow(dead_code)]
impl CamelNetwork {

    fn find_steps_pt1(&self) -> u32 {
        let mut curr = "AAA";
        let mut count = 0;

        for inst in self.inst.chars().cycle() {
            match inst {
                'L' => curr = self.nodes[curr].0.as_str(),
                'R' => curr = self.nodes[curr].1.as_str(),
                _ => panic!("Invalid instruction"),
            }
            count += 1;
            if curr == "ZZZ" {
                return count;
            }
        }
        0
    }

    fn find_steps_pt2(&self) -> u64 {
        let mut counts: Vec<u64> = vec![];
        let mut count = 0u64;

        let mut ghosts =
            self.nodes.keys()
            .filter(|k| k.ends_with('A'))
            .map(|x| x.as_str())
            .collect::<Vec<_>>();

        println!("{:?}", ghosts);

        for inst in self.inst.chars().cycle() {
            count += 1;

            for g in ghosts.iter_mut() {
                if inst == 'L' { *g = self.nodes[*g].0.as_str(); }
                if inst == 'R' { *g = self.nodes[*g].1.as_str(); }

                if (*g).ends_with('Z') {
                    counts.push(count);
                }
            }

            //println!("counts.len()={:?}, ghosts.len()={:?}", counts.len(), ghosts.len());
            if counts.len() >= ghosts.len() {
                return shared::lcm(&counts);
            }
        }
        0
    }
}

impl From<&str> for CamelNetwork {
    fn from(item: &str) -> Self {
        let inst = item.split("\r\n").take(1).map(|x| x.to_owned()).next().unwrap();

        let re2 = Regex::new(r"([0-9A-Z]+) = .([0-9A-Z]+), ([0-9A-Z]+).").unwrap();
        let mut nodes = HashMap::<String, (String, String)>::new();

        re2.captures_iter(item)
            .for_each(|c| {
                let (_, [enode, eleft, eright]) = c.extract();
                nodes.insert(enode.to_owned(), (eleft.to_owned(), eright.to_owned()));
            });

        //println!("{:?}", inst);
        //println!("{:?}", nodes);

        CamelNetwork {
            inst,
            nodes,
        }
    }
}

pub fn part1() -> String {
    let input = include_str!("../input1.txt");

    let net = CamelNetwork::from(input);
    net.find_steps_pt1().to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");

    let net = CamelNetwork::from(input);
    net.find_steps_pt2().to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_sample() {
        let input = include_str!("../input2.txt");

        let net = CamelNetwork::from(input);

        assert_eq!(net.inst, String::from("RL"));

        assert_eq!(net.nodes.keys().len(), 7);
        assert_eq!(net.nodes["AAA"].0, String::from("BBB"));
        assert_eq!(net.nodes["AAA"].1, String::from("CCC"));
        assert_eq!(net.nodes["BBB"].0, String::from("DDD"));
        assert_eq!(net.nodes["BBB"].1, String::from("EEE"));
    }

    #[test]
    fn solve_steps_pt1() {
        let input2 = include_str!("../input2.txt");
        let net2 = CamelNetwork::from(input2);
        assert_eq!(net2.find_steps_pt1(), 2);

        let input3 = include_str!("../input3.txt");
        let net3 = CamelNetwork::from(input3);
        assert_eq!(net3.find_steps_pt1(), 6);
    }

    #[test]
    fn solve_steps_pt2() {
        let input4 = include_str!("../input4.txt");
        let net4 = CamelNetwork::from(input4);
        assert_eq!(net4.find_steps_pt2(), 6);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "18827");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "20220305520997");
    }
}
