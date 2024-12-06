use std::collections::HashSet;

advent_of_code::solution!(6);

enum Direction {
    North,
    East,
    South,
    West,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = HashSet::new();
    let mut start_position = (0, 0);

    let height = input.lines().count() as i32;
    let width = input.lines().next().map_or(0, |line| line.len() as i32);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                map.insert((x as i32, y as i32));
            }
            '^' => {
                start_position = (x as i32, y as i32);
            }
            _ => {}
        });
    });

    let mut visited = HashSet::new();
    visited.insert(start_position);
    let mut current_position = start_position;
    let mut current_direction = Direction::North;

    loop {
        let next_position = match current_direction {
            Direction::North => (current_position.0, current_position.1 - 1),
            Direction::East => (current_position.0 + 1, current_position.1),
            Direction::South => (current_position.0, current_position.1 + 1),
            Direction::West => (current_position.0 - 1, current_position.1),
        };

        if !(0..width).contains(&next_position.0) || !(0..height).contains(&next_position.1) {
            break;
        }

        if map.contains(&next_position) {
            current_direction = match current_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        } else {
            visited.insert(next_position);
            current_position = next_position;
        }
    }

    Some(visited.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
