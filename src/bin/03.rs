advent_of_code::solution!(3);

const MAX_LENGTH: usize = "mul(333,333)".len();

fn is_mul(str: &str) -> Option<(u32, u32)> {
    if !str.starts_with("mul(") {
        return None;
    }

    for (i, c) in str[4..4 + 4].char_indices() {
        if c.is_alphanumeric() {
            continue;
        }
        if c == ',' {
            for (ii, c) in str[5 + i..5 + i + 4].char_indices() {
                if c.is_alphanumeric() {
                    continue;
                }
                if c == ')' {
                    let a = str[4..4 + i].parse().expect("1 Not a number");
                    let b = str[5 + i..5 + i + ii].parse().expect("2 Not a number");
                    return Some((a, b));
                }
                return None;
            }
        }
        return None;
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        (0..=input.len())
            .filter_map(|i| is_mul(&input[i..usize::min(input.len(), i + MAX_LENGTH)]))
            .map(|(a, b)| a * b)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut enabled = true;
    for i in 0..=input.len() {
        let min = usize::min(input.len(), i + MAX_LENGTH);
        let slice = &input[i..min];

        if slice.starts_with("do()") {
            enabled = true;
        } else if slice.starts_with("don't()") {
            enabled = false;
        }

        if enabled {
            if let Some((a, b)) = is_mul(slice) {
                sum += a * b;
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))\n";
        let result = part_two(&str);
        assert_eq!(result, Some(48));
    }
}
