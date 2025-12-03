advent_of_code::solution!(2);

fn pow(n: u64, p: u64) -> u64 {
    let mut res = 1;
    for _ in 0..p {
        res *= n;
    }
    res
}

fn dig(n: u64) -> u64 {
    let mut d = 1;
    let mut n = n;
    loop {
        n /= 10;
        if n == 0 {
            break;
        }
        d += 1;
    }
    d
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for seg in input.split(',') {
        let seg = seg.trim();
        let (lstr, rstr) = seg.split_once('-').expect("incorrect input format");
        let (l, r) = (
            lstr.parse::<u64>().expect("could not parse l boundary"),
            rstr.parse::<u64>().expect("could not parse r boundary"),
        );

        for n in l..=r {
            let d = dig(n);
            let div = pow(10, d / 2);
            if n / div == n % div {
                sum += n;
            }
        }
    }

    Some(sum)
}

fn is_repeating(s: &str, pattern: &str) -> bool {
    for (i, _) in s.chars().enumerate().step_by(pattern.len()) {
        let part: String = s.chars().skip(i).take(pattern.len()).collect();
        if part != pattern {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    for seg in input.split(',') {
        let seg = seg.trim();
        let (lstr, rstr) = seg.split_once('-').expect("incorrect input format");
        let (l, r) = (
            lstr.parse::<u64>().expect("could not parse l boundary"),
            rstr.parse::<u64>().expect("could not parse r boundary"),
        );

        for n in l..=r {
            let nstr = n.to_string();
            for i in 0..nstr.len() / 2 {
                let pattern = nstr.split_at(i + 1).0;
                if is_repeating(&nstr, pattern) {
                    sum += n;
                    break;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
