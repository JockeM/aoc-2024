use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let m = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    const OFFSETS: &[&[(i32, i32)]] = &[
        &[(0, 0), (0, 1), (0, 2), (0, 3)],
        &[(0, 0), (0, -1), (0, -2), (0, -3)],
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        &[(0, 0), (1, 1), (2, 2), (3, 3)],
        &[(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        &[(0, 0), (1, -1), (2, -2), (3, -3)],
        &[(0, 0), (-1, 1), (-2, 2), (-3, 3)],
    ];

    let mut count = 0;
    for (y, line) in m.iter().enumerate() {
        for x in 0..line.len() {
            for offsets in OFFSETS.iter() {
                let s = offsets
                    .iter()
                    .filter_map(|(oy, ox)| {
                        let ny = y as i32 + oy;
                        let nx = x as i32 + ox;
                        if ny.is_negative() || nx.is_negative() {
                            return None;
                        }
                        m.get(ny as usize).and_then(|a| a.get(nx as usize))
                    })
                    .collect::<String>();
                if s == "XMAS" {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    const OFFSETS: &[&[(i32, i32)]] = &[
        &[(-1, -1), (0, 0), (1, 1)],
        &[(-1, 1), (0, 0), (1, -1)],
        &[(1, 1), (0, 0), (-1, -1)],
        &[(1, -1), (0, 0), (-1, 1)],
    ];

    let mut count = 0;
    for (y, line) in m.iter().enumerate() {
        for x in 0..line.len() {
            let p = OFFSETS
                .iter()
                .map(|offsets| {
                    offsets
                        .iter()
                        .filter_map(|(oy, ox)| {
                            let ny = y as i32 + oy;
                            let nx = x as i32 + ox;
                            if ny.is_negative() || nx.is_negative() {
                                return None;
                            }
                            m.get(ny as usize).and_then(|a| a.get(nx as usize))
                        })
                        .collect::<String>()
                })
                .collect_vec();
            let c = p.iter().filter(|s| s.as_str() == "MAS").count();
            if c == 2 {
                count += 1;
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
