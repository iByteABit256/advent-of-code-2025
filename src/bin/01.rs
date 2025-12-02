advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i64 = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let (dir_str, times_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let times = times_str.parse::<i64>().expect("times_str is not a i64");
        dial = if dir == 'L' {
            (dial - times).rem_euclid(100)
        } else {
            (dial + times) % 100
        };

        if dial == 0 {
            zeroes += 1;
        }
    }

    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: i64 = 50;
    let mut zeroes: u64 = 0;

    for line in input.lines() {
        let (dir_str, times_str) = line.split_at(1);
        let dir = dir_str.chars().next().unwrap();
        let times = times_str.parse::<i64>().expect("times_str is not a i64");
        if dir == 'L' {
            zeroes += times as u64 / 100;
            if dial != 0 && dial <= times.rem_euclid(100) {
                zeroes += 1;
            }
            dial = (dial - times).rem_euclid(100);
        } else {
            zeroes += (dial + times) as u64 / 100;
            dial = (dial + times) % 100;
        }
    }

    Some(zeroes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
