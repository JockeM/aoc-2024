use itertools::Itertools;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let mut locks = vec![];
    let mut keys = vec![];

    for group in input.split("\n\n") {
        let lines = group.lines().collect_vec();
        let mut height = vec![-1; lines[0].len()];

        for line in &lines {
            for (i, cell) in line.chars().enumerate() {
                if cell == '#' {
                    height[i] += 1;
                }
            }
        }

        if group.starts_with('#') {
            locks.push(height);
        } else {
            keys.push(height);
        }
    }

    Some(
        locks
            .iter()
            .map(|lock| {
                keys.iter()
                    .filter(|key| lock.iter().zip(key.iter()).all(|(&l, &k)| l + k <= 5))
                    .count()
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
