advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn is_accessible(board: &[Vec<char>], location: (&usize, &usize)) -> bool {
    // just return false for empty locations for simplification
    if board[*location.0][*location.1] != '@' {
        return false;
    }

    let (x, y) = (*location.0 as i32, *location.1 as i32);
    let mut rolls = 0;

    for i in x - 1..=x + 1 {
        if i < 0 || i >= board.len() as i32 {
            continue;
        }

        for j in y - 1..=y + 1 {
            if j < 0 || j >= board[0].len() as i32 {
                continue;
            }

            if i == x && j == y {
                continue;
            }

            if board[i as usize][j as usize] == '@' {
                rolls += 1;
            }

            if rolls == 4 {
                return false;
            }
        }
    }

    true
}

fn get_accessible(board: &[Vec<char>]) -> Vec<(usize, usize)> {
    board
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |(j, _col)| is_accessible(board, (&i, j)))
                .map(move |(j, _col)| (i, j))
        })
        .collect::<Vec<(usize, usize)>>()
}

fn remove_accessible(board: &mut [Vec<char>]) -> u64 {
    let accessible = get_accessible(board);

    accessible.iter().for_each(|loc| {
        board[loc.0][loc.1] = '.';
    });

    accessible.len() as u64
}

fn remove_max(mut board: Vec<Vec<char>>) -> u64 {
    let mut removed = 0;

    loop {
        let removed_part = remove_accessible(&mut board);
        if removed_part == 0 {
            break;
        }
        removed += removed_part;
    }

    removed
}

/*
fn print_board(board: &[Vec<char>]) {
    board
        .iter()
        .for_each(|r| println!("{}", r.iter().collect::<String>()));
}
*/

pub fn part_one(input: &str) -> Option<u64> {
    let board = parse_input(input);

    Some(
        board
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(j, _col)| is_accessible(&board, (&i, j)))
                    .count() as u64
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(remove_max(parse_input(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
