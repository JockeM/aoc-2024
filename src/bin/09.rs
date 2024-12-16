advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Vec::new();
    let mut id = 0;
    for (n, c) in input.chars().filter_map(|c| c.to_digit(10)).enumerate() {
        if n % 2 == 0 {
            disk.extend(std::iter::repeat(id).take(c as usize));
            id += 1;
        } else {
            disk.extend(std::iter::repeat(u32::MAX).take(c as usize));
        }
    }

    let mut last_free = 0;
    while let Some(pos) = disk[last_free..].iter().position(|&d| d == u32::MAX) {
        if let Some(last) = disk.pop() {
            if last != u32::MAX {
                disk[pos + last_free] = last;
            }
        }
        last_free = pos;
    }

    Some(
        disk.iter()
            .enumerate()
            .filter(|&(_, &c)| c != u32::MAX)
            .map(|(i, &c)| i as u64 * c as u64)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
