use std::collections::HashSet;

advent_of_code::solution!(15);

fn move_pos(m: char, position: (usize, usize)) -> (usize, usize) {
    match m {
        '>' => (position.0, position.1 + 1),
        '<' => (position.0, position.1 - 1),
        '^' => (position.0 - 1, position.1),
        _ => (position.0 + 1, position.1),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut boxes = HashSet::new();
    let mut map = HashSet::new();
    let mut position = (0, 0);

    let (map_input, moves_input) = input.split_once("\n\n")?;

    map_input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            match c {
                '#' => {
                    map.insert((y, x));
                }
                'O' => {
                    boxes.insert((y, x));
                }
                '@' => {
                    position = (y, x);
                }
                _ => {}
            };
        });
    });

    'outer: for m in moves_input.chars() {
        if m == '\n' {
            continue;
        }

        let next_move = move_pos(m, position);
        if map.contains(&next_move) {
            continue;
        }

        if boxes.contains(&next_move) {
            let mut test = next_move;
            while !map.contains(&test) {
                if !boxes.contains(&test) {
                    boxes.remove(&next_move);
                    position = next_move;
                    boxes.insert(test);
                    continue 'outer;
                }
                test = move_pos(m, test);
            }
        } else {
            position = next_move;
        }
    }

    Some(boxes.iter().map(|(y, x)| (100 * y + x) as u32).sum())
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
