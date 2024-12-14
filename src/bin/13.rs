advent_of_code::solution!(13);

fn parse_button(str: &str) -> (u64, u64) {
    let mut it = str.split(": X+").last().unwrap().split(", Y+");
    let x = it.next().unwrap().parse().unwrap();
    let y = it.next().unwrap().parse().unwrap();
    (x, y)
}

fn parse_target(str: &str) -> (u64, u64) {
    let mut it = str.split(": X=").last().unwrap().split(", Y=");
    let x = it.next().unwrap().parse().unwrap();
    let y = it.next().unwrap().parse().unwrap();
    (x, y)
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let mut it = group.lines();
                let a = parse_button(it.next().unwrap());
                let b = parse_button(it.next().unwrap());
                let target = parse_target(it.next().unwrap());

                (a, b, target)
            })
            .filter_map(|(a, b, target)| {
                let (ax, ay) = a;
                let (bx, by) = b;
                let (tx, ty) = target;

                let max_na = tx / ax;
                for na in 0..=max_na {
                    let (px, py) = (ax * na, ay * na);
                    if px > tx || py > ty {
                        return None;
                    }

                    let (dx, dy) = (tx - px, ty - py);

                    if dx % bx == 0 && dy % by == 0 {
                        if dx / bx == dy / by {
                            let nb = dx / bx;
                            return Some(na * 3 + nb);
                        }
                    }
                }

                None
            })
            .sum::<u64>(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_button() {
        let input = "Button A: X+85, Y+25";
        let expected = (85, 25);
        assert_eq!(parse_button(input), expected);
    }

    #[test]
    fn test_parse_target() {
        let input = "Prize: X=6657, Y=5481";
        let expected = (6657, 5481);
        assert_eq!(parse_target(input), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
