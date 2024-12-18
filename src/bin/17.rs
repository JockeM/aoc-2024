use itertools::Itertools;

advent_of_code::solution!(17);

fn calc(mut a: i32, mut b: i32, mut c: i32, program: &[i32]) -> String {
    let mut out = vec![];
    let mut pc = 0usize;
    while pc < program.len() - 1 {
        let op = program[pc];
        let literal = program[pc + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => panic!("Invalid"),
        };
        match op {
            0 => a = a / (2_i32.pow(combo as u32)),
            1 => b = b ^ literal,
            2 => b = combo % 8,
            3 if a != 0 => {
                pc = literal as usize;
                continue;
            }
            4 => b = b ^ c,
            5 => out.push(combo % 8),
            6 => b = a / (2_i32.pow(combo as u32)),
            7 => c = a / (2_i32.pow(combo as u32)),
            _ => {}
        }

        pc += 2;
    }

    out.iter().join(",")
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_at(12).1.parse::<i32>().unwrap();
    let b = lines.next().unwrap().split_at(12).1.parse::<i32>().unwrap();
    let c = lines.next().unwrap().split_at(12).1.parse::<i32>().unwrap();
    let program = lines
        .skip(1)
        .next()
        .unwrap()
        .split_at(9)
        .1
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    Some(calc(a, b, c, &program))
}

pub fn part_two(_input: &str) -> Option<i64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
