advent_of_code::solution!(6);

fn solve(numbers: &[Vec<u64>], operators: &[char]) -> u64 {
    let mut sum = 0;
    for (j, _) in operators.iter().enumerate() {
        if operators[j] == '*' {
            sum += numbers
                .iter()
                .enumerate()
                .map(|(i, _)| numbers[i][j])
                .product::<u64>();
        } else if operators[j] == '+' {
            sum += numbers
                .iter()
                .enumerate()
                .map(|(i, _)| numbers[i][j])
                .sum::<u64>();
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines: Vec<&str> = input.lines().collect();
    let operators: Vec<char> = lines
        .pop()
        .expect("input is empty")
        .split_whitespace()
        .map(|s| s.chars().last().expect("operator empty"))
        .collect();

    let numbers: Vec<Vec<u64>> = lines
        .iter()
        .map(|row| {
            row.split_whitespace()
                .map(|s| s.parse::<u64>().expect("not a number"))
                .collect()
        })
        .collect();

    Some(solve(&numbers, &operators))
}

fn reflect(numbers: &[Vec<char>]) -> Vec<String> {
    let mut char_map = vec![vec![' '; numbers.len()]; numbers[0].len()];

    for (i, _row) in numbers.iter().enumerate() {
        for (j, _col) in numbers[i].iter().enumerate() {
            char_map[j][i] = numbers[i][j];
        }
    }

    let c: Vec<String> = char_map
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();

    c
}

fn solve_reflected(numbers: &[String], operators: &[char]) -> u64 {
    let mut sum = 0;

    let mut operator_idx = 0;
    let mut numbers_curr = Vec::new();

    for row in numbers {
        let row = row.trim();

        if row.is_empty() {
            if operators[operator_idx] == '*' {
                sum += numbers_curr.iter().product::<u64>();
            } else if operators[operator_idx] == '+' {
                sum += numbers_curr.iter().sum::<u64>();
            }

            operator_idx += 1;
            numbers_curr.clear();
            continue;
        }

        numbers_curr.push(row.parse::<u64>().expect("not a number"));
    }
    if operators[operator_idx] == '*' {
        sum += numbers_curr.iter().product::<u64>();
    } else if operators[operator_idx] == '+' {
        sum += numbers_curr.iter().sum::<u64>();
    }

    sum
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut lines: Vec<&str> = input.lines().collect();
    let operators: Vec<char> = lines
        .pop()
        .expect("input is empty")
        .split_whitespace()
        .map(|s| s.chars().last().expect("operator empty"))
        .collect();

    let numbers: Vec<Vec<char>> = lines.iter().map(|row| row.chars().collect()).collect();

    Some(solve_reflected(&reflect(&numbers), &operators))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
