pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first_digit = line
                .chars()
                .find(|char| char.is_ascii_digit())
                .expect("No digit found");
            let last_digit = line
                .chars()
                .rfind(|char| char.is_ascii_digit())
                .expect("No digit found");
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
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
            ),
            142
        );
    }
}
