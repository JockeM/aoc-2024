use itertools::Itertools;

advent_of_code::solution!(7);

fn concat(a: u64, b: u64) -> Option<u64> {
    let mut b_digits = 0;
    let mut temp_b = b;
    while temp_b > 0 {
        temp_b /= 10;
        b_digits += 1;
    }
    if b_digits == 0 {
        b_digits = 1;
    }
    Some(a * 10_u64.pow(b_digits) + b)
}

fn makes_sum<F>(values: &[u64], ops: &[F], final_sum: u64) -> Option<u64>
where
    F: Fn(u64, u64) -> Option<u64>,
{
    let mut sum = values[0];

    for (i, &value) in values.iter().enumerate().skip(1) {
        if sum > final_sum {
            return None;
        }

        sum = ops[i - 1](sum, value)?;
    }

    (final_sum == sum).then_some(sum)
}

fn generate_combinations<T>(n: usize, choices: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..n)
        .map(|_| choices.to_vec())
        .multi_cartesian_product()
        .collect()
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

    let choices: Vec<fn(u64, u64) -> Option<u64>> = vec![u64::checked_add, u64::checked_mul];

    Some(
        data.iter()
            .map(|(sum, values)| {
                generate_combinations(values.len() - 1, &choices)
                    .into_iter()
                    .find_map(|ops| makes_sum(&values, &ops, *sum))
                    .unwrap_or(0)
            })
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

    let choices: Vec<fn(u64, u64) -> Option<u64>> =
        vec![u64::checked_add, u64::checked_mul, concat];

    Some(
        data.iter()
            .map(|(sum, values)| {
                generate_combinations(values.len() - 1, &choices)
                    .into_iter()
                    .find_map(|ops| makes_sum(&values, &ops, *sum))
                    .unwrap_or(0)
            })
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
}
