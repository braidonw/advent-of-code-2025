advent_of_code::solution!(1);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Rotation {
    direction: Direction,
    turns: i32,
}

impl Rotation {
    // s could be something like "L90" or "R4"
    fn parse(s: &str) -> Self {
        match s.split_at(1) {
            ("L", turns) => Self {
                direction: Direction::Left,
                turns: turns.parse().unwrap(),
            },
            ("R", turns) => Self {
                direction: Direction::Right,
                turns: turns.parse().unwrap(),
            },
            _ => panic!("Invalid rotation"),
        }
    }
}

fn rotate(start: i32, rotation: &Rotation) -> i32 {
    match rotation.direction {
        Direction::Left => start - rotation.turns,
        Direction::Right => start + rotation.turns,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = input.lines().map(Rotation::parse).collect::<Vec<_>>();
    let mut current = 50;
    let mut count = 0;

    for rotation in rotations {
        current = match rotation.direction {
            Direction::Left => current - rotation.turns,
            Direction::Right => current + rotation.turns,
        };
        if current % 100 == 0 {
            count += 1;
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = input.lines().map(Rotation::parse).collect::<Vec<_>>();
    let mut current = 50;
    let mut count = 0;

    for rotation in rotations {
        current = match rotation.direction {
            Direction::Left => current - rotation.turns,
            Direction::Right => (current + rotation.turns) / 100,
        };
        if current % 100 == 0 {
            count += 1;
        }
    }

    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
