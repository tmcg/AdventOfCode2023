#![allow(dead_code)]

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

struct Hand {
    bid: u64,
    htype: HandType,
    cards: [u32; 5],
}

impl Hand {
    fn strength(&self) -> u64 {
        10000000000 * self.htype as u64 +
        100000000 * self.cards[0] as u64 +
        1000000 * self.cards[1] as u64 +
        10000 * self.cards[2] as u64 +
        100 * self.cards[3] as u64 +
        self.cards[4] as u64
    }

    fn card_value(c: char) -> u32 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap_or(0),
        }
    }

    fn parse_buckets(cards: &str) -> Vec<(u32, u32)> {
        let mut res: [u32; 15] = [0; 15];
    
        for ch in cards.chars() {
            res[Hand::card_value(ch) as usize] += 1;
        }
    
        let mut vres =
            res.iter()
            .enumerate()
            .filter(|&(i,n)| i > 0 && *n > 0)
            .map(|(i,n)| (i as u32, *n))
            .collect::<Vec<_>>();
    
        vres.sort_by(|a, b| {
            if b.1 == a.1 {
                b.0.cmp(&a.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        vres
    }

    fn parse_bid(bid: &str) -> u64 {
        bid.parse::<u64>().expect("Unable to parse bid")
    }
    
    fn parse_type(cards: &str) -> HandType {
        let bkts = Hand::parse_buckets(cards);

        match bkts[0].1 {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => match bkts[1].1 {
                2 => HandType::FullHouse,
                _ => HandType::ThreeOfAKind,
            },
            2 => match bkts[1].1 {
                2 => HandType::TwoPair,
                _ => HandType::OnePair,
            },
            _ => HandType::HighCard,
        }
    }
}

impl From<&str> for Hand {
    fn from(item: &str) -> Self {
        let sp = item.split(' ').collect::<Vec<_>>();
        let sp_bid = Hand::parse_bid(sp[1]);
        let sp_htype = Hand::parse_type(sp[0]);

        let sp_cards: [u32; 5] = 
            sp[0].chars()
            .map(Hand::card_value)
            .collect::<Vec<_>>()
            .try_into().unwrap();

        Hand {
            bid: sp_bid,
            htype: sp_htype,
            cards: sp_cards,
        }
    }
}

fn total_winnings(lines: &[String]) -> u64 {
    let mut hands: Vec<Hand> = 
        lines.iter()
            .map(|x| Hand::from(x.as_str()))
            .collect::<Vec<_>>();

    hands.sort_by(|a,b| {
        let sb = b.strength();
        let sa = a.strength();
        sb.cmp(&sa)
    });

    //for hand in &hands {
    //    println!("{:?} = {:?}", hand.cards, hand.bid);
    //}

    let hlen = hands.len() as u64;

    println!("{:?}", hlen);
    hands.iter()
        .enumerate()
        .map(|(i, x)| (hlen - (i as u64)) * x.bid)
        .sum::<u64>()
}

pub fn part1() -> String {
    let input = include_str!("../input1.txt");
    let lines = shared::input_as_lines(input);

    total_winnings(&lines).to_string()
}

pub fn part2() -> String {
    let input = include_str!("../input1.txt");
    let _lines = shared::input_as_lines(input);

    "".to_string()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_buckets() {
        assert_eq!(Hand::parse_buckets("AAAAA"),
            vec![(14, 5)]);
        assert_eq!(Hand::parse_buckets("1KKKK"),
            vec![(13, 4), (1, 1)]);
        assert_eq!(Hand::parse_buckets("11221"),
            vec![(1, 3), (2, 2)]);
        assert_eq!(Hand::parse_buckets("22111"),
            vec![(1, 3), (2, 2)]);
        assert_eq!(Hand::parse_buckets("11231"),
            vec![(1, 3), (3, 1), (2, 1)]);
        assert_eq!(Hand::parse_buckets("42QQ2"),
            vec![(12, 2), (2, 2), (4, 1)]);
        assert_eq!(Hand::parse_buckets("AKT33"),
            vec![(3, 2), (14, 1), (13, 1), (10, 1)]);
        assert_eq!(Hand::parse_buckets("23857"),
            vec![(8, 1), (7, 1), (5, 1), (3, 1), (2, 1)]);
    }

    #[test]
    fn parse_type() {
        assert_eq!(Hand::parse_type("AAAAA"),
            HandType::FiveOfAKind);
        assert_eq!(Hand::parse_type("1KKKK"),
            HandType::FourOfAKind);
        assert_eq!(Hand::parse_type("11221"),
            HandType::FullHouse);
        assert_eq!(Hand::parse_type("11231"),
            HandType::ThreeOfAKind);
        assert_eq!(Hand::parse_type("42QQ2"),
            HandType::TwoPair);
        assert_eq!(Hand::parse_type("AKT33"),
            HandType::OnePair);
        assert_eq!(Hand::parse_type("23857"),
            HandType::HighCard);
    }

    #[test]
    fn parse_hand() {
        let h1 = Hand::from("AAAAA 123");
        assert_eq!(h1.cards, [14,14,14,14,14]);
        assert_eq!(h1.htype, HandType::FiveOfAKind);
        assert_eq!(h1.bid, 123);
        assert_eq!(h1.strength(), 61414141414);

        let h2 = Hand::from("55QQK 99");
        assert_eq!(h2.cards, [5,5,12,12,13]);
        assert_eq!(h2.htype, HandType::TwoPair);
        assert_eq!(h2.bid, 99);
        assert_eq!(h2.strength(), 20505121213);
    }

    #[test]
    fn parse_sample() {
        let input = include_str!("../input2.txt");
        let lines = shared::input_as_lines(input);
    
        let mut hands: Vec<Hand> = 
            lines.iter()
                .map(|x| Hand::from(x.as_str()))
                .collect::<Vec<_>>();

        hands.sort_by(|a,b| {
            let sb = b.strength();
            let sa = a.strength();
            sb.cmp(&sa)
        });

        assert_eq!(hands.len(), 5);
        assert_eq!(hands[0].strength(), 31212121114);
        assert_eq!(hands[1].strength(), 31005051105);
        assert_eq!(hands[2].strength(), 21313060707);
        assert_eq!(hands[3].strength(), 21310111110);
        assert_eq!(hands[4].strength(), 10302100313);

        assert_eq!(total_winnings(&lines), 6440);
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "251216224");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "zz");
    }
}
