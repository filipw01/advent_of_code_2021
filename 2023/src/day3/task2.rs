#[derive(Debug)]
struct Gear {
    row: usize,
    column: usize,
    numbers: Vec<Number>,
}

impl Gear {
    fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            numbers: vec![],
        }
    }
    fn add_number(&mut self, number: Number) {
        let start = number.column_start.checked_sub(1).unwrap_or(0);
        if (self.row).abs_diff(number.row) > 1
                // at least on of the numbers must be at most 1 column away
                || !(start..=(number.column_end+1)).contains(&self.column)
        {
            return;
        }
        self.numbers.push(number);
    }
}

#[derive(Debug, Clone)]
struct Number {
    column_start: usize,
    column_end: usize,
    row: usize,
    value: usize,
}

impl Number {
    fn new(row: usize, column_start: usize, column_end: usize, value: usize) -> Self {
        Self {
            column_start,
            column_end,
            row,
            value,
        }
    }
}

pub fn solution(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|e| e.chars().collect()).collect();
    let mut gears = lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(column, character)| {
                    if *character == '*' {
                        Some(Gear::new(row, column))
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();

    let mut numbers = vec![];
    for (row, line) in lines.iter().enumerate() {
        let mut active_number = "".to_string();
        for (column, character) in line.iter().enumerate() {
            if character.is_ascii_digit() {
                active_number.push(*character);
            } else {
                let next_char = line.get(column);
                if !active_number.is_empty() && !next_char.unwrap_or(&'.').is_ascii_digit() {
                    numbers.push(Number::new(
                        row,
                        column - active_number.len(),
                        column - 1,
                        active_number.parse::<usize>().unwrap(),
                    ));
                    active_number.clear();
                }
            }
            if column == line.len() - 1 && !active_number.is_empty() {
                numbers.push(Number::new(
                    row,
                    column - active_number.len() + 1,
                    column,
                    active_number.parse::<usize>().unwrap(),
                ));
                active_number.clear();
            }
        }
    }

    for gear in gears.iter_mut() {
        for number in numbers.clone() {
            gear.add_number(number.clone());
        }
    }

    println!("{:?}", numbers);
    println!("{:?}", gears);
    gears
        .iter()
        .filter_map(|e| {
            if e.numbers.len() == 2 {
                return Some(e.numbers[0].value * e.numbers[1].value);
            }
            None
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
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
            ),
            467835
        );
    }

    #[test]
    fn test_edge_case() {
        assert_eq!(
            solution(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
.......755
......*...
.664*598..
"
            ),
            467 * 35 + 755 * 598 + 664 * 598
        );
    }
}
