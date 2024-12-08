use itertools::Itertools;

advent_of_code::solution!(7);

fn concat(a: u64, b: u64) -> u64 {
    let b_digits = (b as f64).log10().floor() as u32 + 1;
    a * 10_u64.pow(b_digits) + b
}

fn can_sum(values: &[u64], ops: &[u8], final_sum: u64) -> bool {
    let mut sum = values[0];

    for i in 1..values.len() {
        let value = values[i];
        sum = match ops[i - 1] {
            0 => sum + value,
            1 => sum * value,
            2 => concat(sum, value),
            _ => unreachable!(),
        };

        if sum > final_sum {
            return false;
        }
    }
    sum == final_sum
}

fn generate_combinations(n: usize, choices: u8) -> impl Iterator<Item = Vec<u8>> {
    (0..n).map(|_| 0..choices).multi_cartesian_product()
}

pub fn part_one(input: &str) -> Option<u64> {
    let data = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let values = b
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            (a.parse::<u64>().unwrap(), values)
        })
        .collect_vec();

    Some(
        data.iter()
            .filter(|(sum, values)| {
                generate_combinations(values.len() - 1, 2)
                    .find(|ops| can_sum(values, &ops, *sum))
                    .is_some()
            })
            .map(|(sum, _)| sum)
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let values = b
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u64>>();
            (a.parse::<u64>().unwrap(), values)
        })
        .collect_vec();

    Some(
        data.iter()
            .filter(|(sum, values)| {
                generate_combinations(values.len() - 1, 3)
                    .find(|ops| can_sum(values, &ops, *sum))
                    .is_some()
            })
            .map(|(sum, _)| sum)
            .sum::<u64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_generate_combinations() {
        let result: Vec<Vec<u8>> = generate_combinations(2, 2).collect();
        assert_eq!(result, vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(12, 3), 123);
        assert_eq!(concat(123, 4), 1234);
    }
}
