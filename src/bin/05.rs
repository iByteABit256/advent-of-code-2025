use std::cmp::{max, min};

advent_of_code::solution!(5);

struct Ingredients {
    fresh: Vec<(u64, u64)>,
    available: Vec<u64>,
}

impl Ingredients {
    fn fresh_count(&self) -> u64 {
        self.fresh.iter().map(|r| r.1 - r.0 + 1).sum()
    }

    fn fresh_and_available(&self) -> u64 {
        let is_fresh = |fruit: &u64| self.fresh.iter().any(|r| *fruit >= r.0 && *fruit <= r.1);

        self.available.iter().filter(|&f| is_fresh(f)).count() as u64
    }
}

fn update_range_if_needed(curr: (u64, u64), new: (u64, u64)) -> Option<(u64, u64)> {
    if new.0 <= curr.1 && new.1 >= curr.0 {
        return Some((min(curr.0, new.0), max(curr.1, new.1)));
    }
    None
}

fn optimize_ranges(ranges: &Vec<(u64, u64)>) -> Option<Vec<(u64, u64)>> {
    let mut ranges_new: Vec<(u64, u64)> = Vec::new();
    let mut updates = 0;

    for r in ranges {
        let mut create_new = true;
        for rn in ranges_new.iter_mut() {
            if let Some(updated) = update_range_if_needed(*rn, *r) {
                *rn = updated;
                updates += 1;
                create_new = false;
            }
        }

        if create_new {
            ranges_new.push(*r);
        }
    }

    if updates == 0 {
        return None;
    }

    Some(ranges_new)
}

fn parse_input(input: &str) -> Ingredients {
    let mut lines_it = input.lines();

    let mut fresh: Vec<(u64, u64)> = Vec::new();
    for fresh_range in lines_it.by_ref() {
        if fresh_range.is_empty() {
            break;
        }

        let (lstr, rstr) = fresh_range.split_once('-').expect("incorrect input format");
        let range = (
            lstr.parse::<u64>()
                .expect("could not parse left range boundary"),
            rstr.parse::<u64>()
                .expect("could not parse right range boundary"),
        );

        let mut create_new = true;
        for f in fresh.iter_mut() {
            if let Some(updated) = update_range_if_needed(*f, range) {
                *f = updated;
                create_new = false;
            }
        }

        if create_new {
            fresh.push(range);
        }
    }

    while let Some(optimized) = optimize_ranges(&fresh) {
        fresh = optimized;
    }

    let available: Vec<u64> = lines_it
        .map(|s| s.parse::<u64>().expect("could not parse ingredient"))
        .collect();

    Ingredients { fresh, available }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_input(input).fresh_and_available())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_input(input).fresh_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
