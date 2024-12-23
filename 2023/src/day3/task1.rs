pub fn solution(input: &str) -> usize {
    let lines: Vec<Vec<char>> = input.lines().map(|e| e.chars().collect()).collect();
    let mut total = 0;
    for (row, line) in lines.iter().enumerate() {
        let mut active_number = "".to_string();
        let mut is_adjacent = false;
        for (column, character) in line.iter().enumerate() {
            if character.is_ascii_digit() {
                active_number.push(*character);
                if !is_adjacent {
                    for row_diff in -1..=1 {
                        for column_diff in -1..=1 {
                            let r = row as i32 + row_diff;
                            let c = column as i32 + column_diff;
                            if row_diff == 0 && column_diff == 0 || r < 0 || c < 0 {
                                continue;
                            }
                            if let Some(a) = lines.get(r as usize).and_then(|e| e.get(c as usize)) {
                                if !a.is_ascii_digit() && *a != '.' {
                                    is_adjacent = true;
                                }
                            }
                        }
                    }
                }
            } else {
                if is_adjacent {
                    total += active_number.parse::<usize>().unwrap_or(0);
                }
                active_number.clear();
                is_adjacent = false;
            }
            if column == line.len() - 1 && is_adjacent {
                total += active_number.parse::<usize>().unwrap_or(0);
                active_number.clear();
                is_adjacent = false;
            }
        }
    }
    total
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
            4361
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
            4361
        );
    }
}
