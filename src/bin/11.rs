use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn solve(stone: u64, blink: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if blink == 0 {
        return 1;
    }

    if let Some(&result) = cache.get(&(stone, blink)) {
        return result;
    }

    let result = match stone {
        0 => solve(1, blink - 1, cache),
        s if s.to_string().len() % 2 == 0 => {
            let stone_str = s.to_string();
            let (left, right) = stone_str.split_at(stone_str.len() / 2);

            solve(left.parse().unwrap(), blink - 1, cache)
                + solve(right.parse().unwrap(), blink - 1, cache)
        }
        s => solve(s * 2024, blink - 1, cache),
    };

    cache.insert((stone, blink), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::<(u64, usize), u64>::new();
    let mut sum = 0;
    for stone in stones {
        sum += solve(stone, 25, &mut cache);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::<(u64, usize), u64>::new();
    let mut sum = 0;
    for stone in stones {
        sum += solve(stone, 75, &mut cache);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
