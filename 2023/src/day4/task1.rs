use std::collections::HashSet;

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let card = Card::from(line);
            card.get_score()
        })
        .sum()
}

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl Card {
    fn get_score(&self) -> usize {
        let matching_numbers = self.winning_numbers.intersection(&self.numbers).count();
        if matching_numbers > 0 {
            return 2usize.pow(matching_numbers as u32 - 1);
        }
        0
    }
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, all_numbers) = value[5..].split_once(':').unwrap();
        let (winning_numbers, numbers) = all_numbers.split_once('|').unwrap();
        Self {
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
            13
        );
    }
}
