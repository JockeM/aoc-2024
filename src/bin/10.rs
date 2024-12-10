use std::{collections::HashSet, ops::Deref};

use itertools::Itertools;

advent_of_code::solution!(10);

const OFFSETS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn walk(
    pos: (usize, usize),
    current_level: u8,
    map: &Vec<Vec<u8>>,
    found: &mut Vec<(usize, usize)>,
) {
    for offsets in OFFSETS {
        let new_pos = (
            (pos.0 as i32 + offsets[0]) as usize,
            (pos.1 as i32 + offsets[1]) as usize,
        );
        if let Some(&level) = map
            .get(new_pos.0)
            .and_then(|row| row.deref().get(new_pos.1))
        {
            if current_level == 8 && level == 9 {
                found.push(new_pos);
            }
            if level == (current_level + 1) {
                walk(new_pos, level, map, found)
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();

    let starting = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((y, x), c)))
        .filter(|(_, c)| **c == 0)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (pos, _) in starting {
        let mut found = Vec::new();
        walk(pos, 0, &map, &mut found);
        let h = found.iter().collect::<HashSet<_>>();
        sum += h.len() as u32;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();

    let starting = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| ((y, x), c)))
        .filter(|(_, c)| **c == 0)
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (pos, _) in starting {
        let mut found = Vec::new();
        walk(pos, 0, &map, &mut found);
        sum += found.len() as u32;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
