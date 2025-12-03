advent_of_code::solution!(3);

fn parse_bank(bank_str: &str) -> Vec<u64> {
    bank_str
        .chars()
        .map(|c| c.to_digit(10).expect("incorrect input format") as u64)
        .collect()
}

fn retrieve_batteries(bank: Vec<u64>, n: usize) -> u64 {
    // Vector of max value batteries
    let mut btr_max = vec![0; n];

    for i in 0..bank.len() {
        let btr = bank[i];

        // Only update the max values that fit from the remaining bank
        let start = if bank.len() - i < n {
            n - (bank.len() - i)
        } else {
            0
        };

        // Update the max values
        for j in start..n {
            if btr > btr_max[j] {
                btr_max[j] = btr;

                // If a max value was updated, reset all the following ones
                for btr_to_reset in btr_max.iter_mut().take(n).skip(j + 1) {
                    *btr_to_reset = 0;
                }
                break;
            }
        }
    }

    // Create concatenated form
    let mut batteries = 0;
    let mut pow = 1;
    for i in (0..n).rev() {
        batteries += btr_max[i] * pow;
        pow *= 10;
    }
    batteries
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| retrieve_batteries(parse_bank(bank), 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|bank| retrieve_batteries(parse_bank(bank), 12))
            .sum(),
    )
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
        assert_eq!(result, Some(3121910778619));
    }
}
