struct Digit {
    name: &'static str,
    value: usize,
}
pub fn solution(input: &str) -> usize {
    let valid_digits = [
        Digit {
            name: "one",
            value: 1,
        },
        Digit {
            name: "two",
            value: 2,
        },
        Digit {
            name: "three",
            value: 3,
        },
        Digit {
            name: "four",
            value: 4,
        },
        Digit {
            name: "five",
            value: 5,
        },
        Digit {
            name: "six",
            value: 6,
        },
        Digit {
            name: "seven",
            value: 7,
        },
        Digit {
            name: "eight",
            value: 8,
        },
        Digit {
            name: "nine",
            value: 9,
        },
    ];
    input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            while !valid_digits.iter().any(|d| line.starts_with(d.name))
                && !line.starts_with(|char: char| char.is_ascii_digit())
            {
                line.remove(0);
            }

            while !valid_digits.iter().any(|d| line.ends_with(d.name))
                && !line.ends_with(|char: char| char.is_ascii_digit())
            {
                line.pop();
            }
            let first_digit = valid_digits
                .iter()
                .find(|d| line.starts_with(d.name))
                .map(|d| d.value)
                .unwrap_or_else(|| {
                    line.chars()
                        .next()
                        .expect("should not be empty")
                        .to_digit(10)
                        .expect("should be digit") as usize
                });
            let last_digit = valid_digits
                .iter()
                .find(|d| line.ends_with(d.name))
                .map(|d| d.value)
                .unwrap_or_else(|| {
                    line.chars()
                        .last()
                        .expect("should not be empty")
                        .to_digit(10)
                        .expect("should be digit") as usize
                });
            format!("{}{}", first_digit, last_digit)
                .parse::<usize>()
                .expect("Failed to parse number")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
            ),
            281
        );
    }
}
