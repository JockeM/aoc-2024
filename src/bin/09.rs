advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let mut is_data = true;
    let mut data = Vec::new();
    let mut n = 0;
    for c in input.chars() {
        let count = c.to_digit(10).unwrap();
        for _ in 0..n
        {
            if is_data {
                data.push(n);
            } else {
                data.push(0)
            }
        }


    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
