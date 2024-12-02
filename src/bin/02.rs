use itertools::Itertools;

advent_of_code::solution!(2);

fn valid(parts: &[i32]) -> bool {
    parts
        .windows(2)
        .all(|a| (a[0] - a[1]) > 0 && (a[0] - a[1]) <= 3)
        || parts
            .windows(2)
            .all(|a| (a[0] - a[1]) < 0 && (a[0] - a[1]) >= -3)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec()
            })
            .filter(|parts| valid(parts))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .filter(|parts| {
                valid(parts)
                    || (0..parts.len()).any(|n| {
                        let mut p = parts.clone();
                        p.remove(n);
                        valid(&p)
                    })
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
