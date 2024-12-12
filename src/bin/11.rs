use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn solve(stone: u64, blink: usize, cache: &mut HashMap<u64, u64>) -> u64 {
    if blink == 0 {
        return 1;
    }

    let key = (stone << 7) | (blink as u64);

    if let Some(&result) = cache.get(&key) {
        return result;
    }

    let result = if stone == 0 {
        solve(1, blink - 1, cache)
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            let (left, right) = stone_str.split_at(stone_str.len() / 2);

            solve(left.parse().unwrap(), blink - 1, cache)
                + solve(right.parse().unwrap(), blink - 1, cache)
        } else {
            solve(stone * 2024, blink - 1, cache)
        }
    };

    cache.insert(key, result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();
    Some(
        stones
            .iter()
            .map(|&stone| solve(stone, 25, &mut cache))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = input
        .split_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();
    Some(
        stones
            .iter()
            .map(|&stone| solve(stone, 75, &mut cache))
            .sum(),
    )
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
