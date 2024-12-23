use std::cmp::Ordering;

struct Game {
    id: usize,
    max_red: usize,
    max_green: usize,
    max_blue: usize,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        let (id, rest) = value[5..].split_once(": ").expect("Missing ': '");
        for game in rest.split("; ") {
            let (red, green, blue) = game.split(", ").fold((0, 0, 0), |acc, el| {
                let (value, color) = el.split_once(' ').expect("Entry missing ' '");
                match color {
                    "red" => (value.parse::<usize>().expect("Invalid value"), acc.1, acc.2),
                    "green" => (acc.0, value.parse::<usize>().expect("Invalid value"), acc.2),
                    "blue" => (acc.0, acc.1, value.parse::<usize>().expect("Invalid value")),
                    _ => panic!("Invalid color"),
                }
            });
            max_red = max_red.max(red);
            max_green = max_green.max(green);
            max_blue = max_blue.max(blue);
        }
        Self {
            id: id.parse::<usize>().expect("Invalid id"),
            max_red,
            max_blue,
            max_green,
        }
    }
}

impl PartialEq<Self> for Game {
    fn eq(&self, other: &Self) -> bool {
        self.max_red == other.max_red
            && self.max_green == other.max_green
            && self.max_blue == other.max_blue
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.max_red <= other.max_red
            && self.max_green <= other.max_green
            && self.max_blue <= other.max_blue
        {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
pub fn solution(input: &str) -> usize {
    let comparison_game = Game::from("Game 0: 12 red, 13 green, 14 blue");
    input
        .lines()
        .map(Game::from)
        .filter(|game| game.le(&comparison_game))
        .map(|game| game.id)
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
            ),
            8
        );
    }
}
