
struct BoatRace {
    time: i64,
    dist: i64,
}

impl BoatRace {
    fn find_wins(&self) -> i64 {
        let mut win: i64 = 0;
        for i in 1..self.time {
            let dist = i * (self.time - i);
            if dist > self.dist { win += 1; }
        }
        win
    }
}

pub fn part1() -> String {
    let races = vec![
        BoatRace { time: 49, dist: 356, },
        BoatRace { time: 87, dist: 1378, },
        BoatRace { time: 78, dist: 1502, },
        BoatRace { time: 95, dist: 1882, },
    ];

    let result: i64 = 
        races.iter()
        .map(|x| x.find_wins())
        .product();

    result.to_string()
}

pub fn part2() -> String {
    let race = BoatRace { 
        time: 49877895,
        dist: 356137815021882,
    };

    race.find_wins().to_string()
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
        let races = vec![
            BoatRace { time: 7, dist: 9, },
            BoatRace { time: 15, dist: 40, },
            BoatRace { time: 30, dist: 200, },
        ];

        assert_eq!(races.len(), 3);
        assert_eq!(races[0].time, 7);
        assert_eq!(races[0].dist, 9);

        assert_eq!(races[0].find_wins(), 4);
        assert_eq!(races[1].find_wins(), 8);
        assert_eq!(races[2].find_wins(), 9);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "503424");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "32607562");
    }
}
