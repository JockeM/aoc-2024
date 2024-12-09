use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

fn inside(cord: (i32, i32), width: usize, height: usize) -> bool {
    cord.0 >= 0 && cord.1 >= 0 && cord.0 < width as i32 && cord.1 < height as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next()?.len();
    let height = input.lines().count();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some((c, (x, y)))
                    }
                },
            )
        })
        .into_group_map();

    let mut antinodes = HashSet::new();

    for combos in map.values() {
        for pair in combos.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let delta = ((a.0 as i32 - b.0 as i32), (a.1 as i32 - b.1 as i32));

            let b1 = (delta.0 + a.0 as i32, delta.1 + a.1 as i32);
            let a1 = (-delta.0 + b.0 as i32, -delta.1 + b.1 as i32);

            if inside(a1, width, height) {
                antinodes.insert(a1);
            }
            if inside(b1, width, height) {
                antinodes.insert(b1);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next()?.len();
    let height = input.lines().count();

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(
                move |(x, c)| {
                    if c == '.' {
                        None
                    } else {
                        Some((c, (x, y)))
                    }
                },
            )
        })
        .into_group_map();

    let mut antinodes = HashSet::new();

    for combos in map.values() {
        for pair in combos.iter().combinations(2) {
            let (a, b) = (pair[0], pair[1]);
            let delta = ((a.0 as i32 - b.0 as i32), (a.1 as i32 - b.1 as i32));

            antinodes.insert((a.0 as i32, a.1 as i32));
            antinodes.insert((b.0 as i32, b.1 as i32));

            let mut a1 = (-delta.0 + b.0 as i32, -delta.1 + b.1 as i32);
            while inside(a1, width, height) {
                antinodes.insert(a1);
                a1 = (-delta.0 + a1.0, -delta.1 + a1.1);
            }

            let mut b1 = (delta.0 + a.0 as i32, delta.1 + a.1 as i32);
            while inside(b1, width, height) {
                antinodes.insert(b1);
                b1 = (delta.0 + b1.0, delta.1 + b1.1);
            }
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
