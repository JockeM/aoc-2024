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
                (
                    parse_button(it.next().unwrap()),
                    parse_button(it.next().unwrap()),
                    parse_target(it.next().unwrap()),
                )
            })
            .filter_map(|(a, b, target)| binary_search(a, b, target))
            .map(|(a, b)| a * 3 + b)
            .sum::<u64>(),
    )
}

fn binary_search(
    (ax, ay): (u64, u64),
    (bx, by): (u64, u64),
    (tx, ty): (u64, u64),
) -> Option<(u64, u64)> {
    let a_bigger_than_b = (ty * ax) > (tx * ay);

    let (mut low, mut high) = (0, u64::min(tx / ax, ty / ay));
    while high >= low {
        let mid = (low + high) / 2;
        let na = mid;
        let (px, py) = (ax * na, ay * na);
        if px > tx || py > ty {
            if a_bigger_than_b {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
            continue;
        }
        let (dx, dy) = (tx - px, ty - py);

        let gb = by as u128 * dx as u128;
        let gd = bx as u128 * dy as u128;
        if gb == gd {
            if dx % bx == 0 && dy % by == 0 {
                let nb = dx / bx;
                return Some((na, nb));
            }
            return None;
        } else if gb > gd {
            if a_bigger_than_b {
                low = mid + 1;
            } else {
                if mid == 0 {
                    return None;
                }
                high = mid - 1;
            }
        } else if gb < gd {
            if a_bigger_than_b {
                if mid == 0 {
                    return None;
                }
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(
        input
            .split("\n\n")
            .map(|group| {
                let mut it = group.lines();
                let a = parse_button(it.next().unwrap());
                let b = parse_button(it.next().unwrap());
                let target = parse_target(it.next().unwrap());
                (a, b, (target.0 + 10000000000000, target.1 + 10000000000000))
            })
            .filter_map(|(a, b, target)| binary_search(a, b, target))
            .map(|(a, b)| a as u128 * 3 + b as u128)
            .sum::<u128>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        assert_eq!(binary_search((1, 2), (1, 1), (10, 10)), Some((0, 10)));
        assert_eq!(binary_search((4, 4), (1, 2), (5, 6)), Some((1, 1)));
        assert_eq!(binary_search((8, 4), (2, 6), (10, 10)), Some((1, 1)));
    }

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
        assert_eq!(result, Some(875318608908));
    }
}
