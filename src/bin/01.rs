use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(a, b)| {
            (
                a.trim().parse::<i32>().unwrap(),
                b.trim().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    Some(
        a.iter()
            .sorted()
            .rev()
            .zip(b.iter().sorted().rev())
            .map(|(a, b)| (*a - *b).unsigned_abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|s| s.split_once(' '))
        .map(|(a, b)| {
            (
                a.trim().parse::<u32>().unwrap(),
                b.trim().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let b_counts = b.iter().counts();
    Some(
        a.iter()
            .map(|x| b_counts.get(x).copied().unwrap_or(0) as u32 * x)
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
