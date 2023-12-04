use crate::shared;

const DAY_NUMBER: i32 = 4;

#[derive(Debug)]
struct Card {
    id: u32,
    winners: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> Vec<u32> {
        self.numbers.iter().filter(|x| self.winners.contains(x)).cloned().collect::<Vec<_>>()
    }

    fn points(&self) -> u64 {
        let m = self.matches();
        if !m.is_empty() {
            return u64::pow(2, (m.len() - 1) as u32); 
        }
        0
    }

    fn wins(&self, cards: &Vec<Card>, memo: &mut Vec<i32>) -> u32 {
        let card_id = self.id as usize;
        
        //println!["card id={card_id}"];
        if memo[card_id] < 0 {
            //println!("card {} - calculating", card_id);
            let mut w: u32 = 1;
            let mlen = self.matches().len() as u32;
            if mlen > 0 {
                let m_start = self.id + 1;
                let m_finish = self.id + 1 + mlen;
                for i in m_start..m_finish {
                    w += cards[(i - 1) as usize].wins(cards, memo)
                }
            }
            memo[card_id] = w as i32;
        }

        //println!("card {} result = {}", card_id, memo[card_id] as u64);
        memo[card_id] as u32
    }

    fn create_memo(len: usize) -> Vec<i32> {
        (0..(len + 1)).map(|_| -1).collect::<Vec<_>>()
    }
}

impl From<&str> for Card {
    fn from(item: &str) -> Self {
        const R1: char = ':';
        const R2: char = ' ';

        let s = item.replace(R1, "|");
        let parts = s.split('|').collect::<Vec<_>>();

        let id: u32 = parts.first().unwrap().trim().replace("Card", "").replace(R2, "").parse::<u32>().unwrap();
        let winners: Vec<u32> = parts.get(1).unwrap().trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let numbers: Vec<u32> = parts.get(2).unwrap().trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();

        Card {
            id,
            winners,
            numbers,
        }
    }
}

pub fn part1() -> String {
    let lines = shared::input_file_as_lines(DAY_NUMBER);
    let cards: Vec<Card> = lines.into_iter().map(|x| x.as_str().into()).collect();

    cards.into_iter().map(|x| x.points()).sum::<u64>().to_string()
}

pub fn part2() -> String {
    let lines = shared::input_file_as_lines(DAY_NUMBER);
    let cards: Vec<Card> = lines.into_iter().map(|x| x.as_str().into()).collect();
    let mut memo = Card::create_memo(cards.len());

    let mut total: u32 = 0;
    for card in &cards {
        total += card.wins(&cards, &mut memo);
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input1() {
        let card: Card = 
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".into();

        assert_eq!(card.id, 1);
        assert_eq!(card.winners, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53])
    }

    #[test]
    fn solve_sample1() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",          
        ];

        let cards: Vec<Card> = lines.into_iter().map(|x| x.into()).collect();

        assert_eq![cards[0].points(), 8];
        assert_eq![cards[1].points(), 2];
        assert_eq![cards[2].points(), 2];
        assert_eq![cards[3].points(), 1];
        assert_eq![cards[4].points(), 0];
        assert_eq![cards[5].points(), 0];
    }

    #[test]
    fn solve_sample2() {
        let lines = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",          
        ];

        let cards: Vec<Card> = lines.into_iter().map(|x| x.into()).collect();
        let mut memo = Card::create_memo(cards.len());

        let c6 = cards[5].wins(&cards, &mut memo);
        assert_eq![c6, 1];
        let c5 = cards[4].wins(&cards, &mut memo);
        assert_eq![c5, 1];
        let c4 = cards[3].wins(&cards, &mut memo);
        assert_eq![c4, 2];
        let c3 = cards[2].wins(&cards, &mut memo);
        assert_eq![c3, 4];
        let c2 = cards[1].wins(&cards, &mut memo);
        assert_eq![c2, 7];
        let c1 = cards[0].wins(&cards, &mut memo);
        assert_eq![c1, 15];
    }

    #[test]
    fn solve_part1() {
        assert_eq!(part1(), "28538");
    }

    #[test]
    fn solve_part2() {
        assert_eq!(part2(), "9425061");
    }
}
