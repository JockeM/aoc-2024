use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let rules = input
        .lines()
        .map_while(|line| line.split_once('|'))
        .map(|(left, right)| {
            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();
            (right, left)
        })
        .into_group_map();

    let split_index = input.find("\n\n").unwrap();
    let remaining_input = &input[split_index + 2..];

    Some(
        remaining_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect_vec()
            })
            .filter_map(|row| {
                let mut banned = HashSet::<u32>::new();
                for c in &row {
                    if banned.contains(c) {
                        return None;
                    }
                    if let Some(new_banned) = rules.get(c) {
                        banned.extend(new_banned);
                    }
                }
                Some(row[row.len() / 2])
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules: HashMap<u32, Vec<u32>> = input
        .lines()
        .map_while(|line| line.split_once('|'))
        .map(|(left, right)| (right.parse().expect("Invalid right value"), left.parse().expect("Invalid left value")))
        .into_group_map();

    let remaining_input = input.split("\n\n").nth(1)?;

    Some(
        remaining_input
            .lines()
            .map(|line| {
                line.split(',')
                    .map(|c| c.parse::<u32>().expect("Invalid number"))
                    .collect_vec()
            })
            .filter_map(|mut row| {
                let mut wrong = false;
                let mut i = 0;
                while i < row.len() {
                    if let Some(precedence) = rules.get(&row[i]) {
                        if let Some(last_pos) = row.iter().rposition(|x| precedence.contains(x)) {
                            if last_pos > i {
                                row.swap(last_pos, i);
                                if i > 0 {
                                    i -= 1;
                                }
                                wrong = true;
                                continue;
                            }
                        }
                    }
                    i += 1;
                }
                wrong.then(|| row[row.len() / 2])
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
