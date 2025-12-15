use std::collections::HashSet;

advent_of_code::solution!(8);

fn dist(p: &[u64], q: &[u64]) -> u64 {
    if p.len() != q.len() {
        panic!("invalid input, all points need to have the same dimensions");
    }

    // Squared Eucledean distance
    (0..p.len())
        .map(|i| p[i].abs_diff(q[i]))
        .map(|d| d * d)
        .sum()
}

fn distance_matrix(points: &[Vec<u64>]) -> Vec<Vec<u64>> {
    let n = points.len();
    let mut m = vec![vec![0; n]; n];

    (0..n).for_each(|i| {
        for j in (i + 1)..n {
            let d = dist(&points[i], &points[j]);
            m[i][j] = d;
            m[j][i] = d;
        }
    });

    m
}

fn minimum_distances(m: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let n = m.len();
    let mut pairs: Vec<(usize, usize)> = (0..n)
        .flat_map(|i| (i + 1..n).map(|j| (i, j)).collect::<Vec<(usize, usize)>>())
        .collect();
    pairs.sort_by_key(|&(i, j)| m[i][j]);

    pairs
}

type Circuit = HashSet<usize>;

/*
fn print_matrix(m: &[Vec<u64>]) {
    println!("Distance matrix:\n");
    m.iter().for_each(|row| {
        row.iter().for_each(|col| print!("{} ", col));
        println!();
    });
}

fn to_string(c: &Circuit) -> String {
    let mut s = "{".to_owned();
    c.iter().for_each(|p| s.push_str(&(p.to_string() + ",")));
    s.push('}');
    s
}

fn print_all(circuits: &[Circuit]) {
    circuits.iter().for_each(|c| println!("{}", to_string(c)));
}
*/

fn connect(c1: &Circuit, c2: &Circuit) -> Circuit {
    c1.union(c2).map(|p| p.to_owned()).collect()
}

fn calculate_circuits(m: &[Vec<u64>], num: u64) -> u64 {
    let n = m.len();

    let mut circuits: Vec<Circuit> = (0..n).map(|i| HashSet::from([i])).collect();
    let sorted_distances = minimum_distances(m);

    let mut connected = 0;

    for (i, j) in sorted_distances {
        if connected == num - 1 {
            break;
        }

        let l = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&i))
            .map(|(idx, _)| idx)
            .next()
            .unwrap_or_else(|| panic!("no circuit found for point {}", i));
        let r = circuits
            .iter()
            .enumerate()
            .filter(|(_, c)| c.contains(&j))
            .map(|(idx, _)| idx)
            .next()
            .unwrap_or_else(|| panic!("no circuit found for point {}", j));

        if l == r {
            connected += 1;
            continue;
        }

        let c_new = connect(&circuits[l], &circuits[r]);

        if l < r {
            circuits.remove(r);
            circuits.remove(l);
        } else {
            circuits.remove(l);
            circuits.remove(r);
        }
        circuits.push(c_new);
        connected += 1;
    }

    circuits.sort_by_key(|c| c.len());
    circuits
        .iter()
        .rev()
        .take(3)
        .map(|c| c.len() as u64)
        .product()
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Vec<u64>> = input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse::<u64>().expect("not a number"))
                .collect()
        })
        .collect();

    let m = distance_matrix(&points);

    Some(calculate_circuits(&m, 1000))
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
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
