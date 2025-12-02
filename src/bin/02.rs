advent_of_code::solution!(2);

fn is_invalid(id: u64) -> bool {
    let mut digits = id.to_string().chars().collect::<Vec<char>>();
    let len = digits.len();
    if len % 2 != 0 {
        return false;
    }
    let (left, right) = digits.split_at_mut(len / 2);

    left.iter().zip(right.iter()).all(|(l, r)| l == r)
}

fn is_invalid_p2(id: u64) -> bool {
    // figure it out later :shrug:
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(',').map(|range| {
        let parts: Vec<u64> = range
            .trim()
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    });
    let mut count = 0;

    for (from, to) in ranges {
        for id in from..=to {
            if is_invalid(id) {
                count += id;
            }
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
