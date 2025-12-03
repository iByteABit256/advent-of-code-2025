use std::cmp::max;

advent_of_code::solution!(3);

fn parse_bank(bank_str: &str) -> Vec<u64> {
    bank_str
        .chars()
        .map(|c| c.to_digit(10).expect("incorrect input format") as u64)
        .collect()
}

fn retrieve_batteries(bank: Vec<u64>) -> u64 {
    let mut btr_max = 0;
    let mut btr_max2 = 0;
    for i in 0..bank.len() - 1 {
        let btr = bank[i];
        if btr > btr_max {
            btr_max2 = 0;
            btr_max = btr;
        } else if btr > btr_max2 {
            btr_max2 = btr;
        }
    }
    btr_max2 = max(btr_max2, bank[bank.len() - 1]);
    // println!("result = {}{}", btr_max, btr_max2);
    btr_max * 10 + btr_max2
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| retrieve_batteries(parse_bank(bank)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
