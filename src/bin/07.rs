use std::collections::HashMap;
use std::collections::HashSet;

advent_of_code::solution!(7);

/*
fn print_board(board: &[Vec<char>]) {
    board
        .iter()
        .for_each(|row| println!("{}", row.iter().collect::<String>()));
}
*/

fn in_bounds(board: &[Vec<char>], loc: &(i32, i32)) -> bool {
    let (x, y) = *loc;

    if x < 0 || x >= board.len() as i32 {
        return false;
    }

    if y < 0 || y >= board[0].len() as i32 {
        return false;
    }

    true
}

fn replace_at(board: &mut [Vec<char>], loc: &(i32, i32)) {
    let (x, y) = *loc;

    let x = x as usize;
    let y = y as usize;

    if board[x][y] == '.' {
        board[x][y] = '|';
    }
}

fn beam_split(board: &mut [Vec<char>], pretty: bool) -> u64 {
    //println!("board before splits:");
    //print_board(board);

    let mut splits = 0;

    let mut beams: HashSet<usize> = HashSet::new();

    let entrance = board[0]
        .iter()
        .enumerate()
        .filter(|(_i, c)| **c == 'S')
        .map(|(i, _c)| i)
        .next()
        .expect("no beam entrance found");

    beams.insert(entrance);

    for i in 1..board.len() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for idx in &beams {
            if board[i][*idx] == '^' {
                splits += 1;

                let down = i as i32 + 1;
                let l = *idx as i32 - 1;
                let r = *idx as i32 + 1;

                if in_bounds(board, &(down, l)) {
                    replace_at(board, &(down, l));
                    new_beams.insert(l as usize);
                }
                if in_bounds(board, &(down, r)) {
                    replace_at(board, &(down, r));
                    new_beams.insert(r as usize);
                }
            } else {
                if pretty {
                    let down = i as i32 + 1;
                    if in_bounds(board, &(down, *idx as i32)) {
                        replace_at(board, &(down, *idx as i32));
                    }
                }
                new_beams.insert(*idx);
            }
        }
        beams = new_beams;
        //println!("board after {} iteration:", i);
        //print_board(board);
    }

    splits
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut board: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    Some(beam_split(&mut board, false))
}

fn quantum_split(board: &mut [Vec<char>], pretty: bool) -> u64 {
    //println!("board before splits:");
    //print_board(board);

    let mut splits = 0;

    let mut beams: HashMap<usize, u64> = HashMap::new();

    let entrance = board[0]
        .iter()
        .enumerate()
        .filter(|(_i, c)| **c == 'S')
        .map(|(i, _c)| i)
        .next()
        .expect("no beam entrance found");

    beams.insert(entrance, 1);

    for i in 1..board.len() {
        let mut new_beams: HashMap<usize, u64> = HashMap::new();
        for (idx, timelines) in &beams {
            if board[i][*idx] == '^' {
                splits += timelines;

                let down = i as i32 + 1;
                let l = *idx as i32 - 1;
                let r = *idx as i32 + 1;

                if in_bounds(board, &(down, l)) {
                    replace_at(board, &(down, l));
                    match new_beams.get(&(l as usize)) {
                        Some(new_timelines) => {
                            new_beams.insert(l as usize, new_timelines + timelines);
                        }
                        None => {
                            new_beams.insert(l as usize, *timelines);
                        }
                    }
                    //new_beams.insert(l as usize, timelines + 1);
                }
                if in_bounds(board, &(down, r)) {
                    replace_at(board, &(down, r));
                    match new_beams.get(&(r as usize)) {
                        Some(new_timelines) => {
                            new_beams.insert(r as usize, new_timelines + timelines);
                        }
                        None => {
                            new_beams.insert(r as usize, *timelines);
                        }
                    }
                    //new_beams.insert(r as usize, timelines + 1);
                }
            } else {
                if pretty {
                    let down = i as i32 + 1;
                    if in_bounds(board, &(down, *idx as i32)) {
                        replace_at(board, &(down, *idx as i32));
                    }
                }
                match new_beams.get(idx) {
                    Some(new_timelines) => {
                        new_beams.insert(*idx, new_timelines + timelines);
                    }
                    None => {
                        new_beams.insert(*idx, *timelines);
                    }
                }
                //new_beams.insert(*idx, *timelines);
            }
        }
        beams = new_beams;
        //println!("board after {} iteration:", i);
        //print_board(board);
    }

    splits
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut board: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    Some(quantum_split(&mut board, false) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
