use std::collections::{HashMap, HashSet};

pub fn solution(input: &str) -> usize {
    let mut numbers_of_cards: HashMap<usize, usize> = HashMap::new();
    let max_id = input.lines().count();
    input
        .lines()
        .map(|line| {
            let card = Card::from(line);
            let score = card.get_score();
            let card_count = numbers_of_cards.get(&card.id).unwrap_or(&0) + 1;
            numbers_of_cards.insert(card.id, card_count);
            for i in 1..=score {
                let id = card.id + i;
                if id <= max_id {
                    let value = numbers_of_cards.get(&id).unwrap_or(&0);
                    numbers_of_cards.insert(id, value + card_count);
                }
            }
        })
        .count();
    numbers_of_cards.values().sum()
}

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
    id: usize,
}

impl Card {
    fn get_score(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (id, all_numbers) = value[5..].split_once(':').unwrap();
        let (winning_numbers, numbers) = all_numbers.split_once('|').unwrap();
        Self {
            id: id.trim().parse().unwrap(),
            numbers: numbers
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect(),
            winning_numbers: winning_numbers
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            ),
            30
        );
    }
}
