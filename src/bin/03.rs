use std::str::FromStr;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Bank {
    batteries: [u8; 15],
}

impl FromStr for Bank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut batteries = [0; 15];
        for (i, c) in s.chars().enumerate() {
            batteries[i] = c
                .to_digit(10)
                .ok_or_else(|| "Invalid character".to_string())? as u8;
        }
        Ok(Bank { batteries })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let banks = input
        .lines()
        .map(|line| {
            let bank = line.parse::<Bank>().unwrap();
            dbg!(&bank);
            bank
        })
        .collect::<Vec<_>>();

    let mut max_joltage = 0;

    for bank in &banks {
        let max_pair = {
            let mut pairs: Vec<u64> = Vec::new();
            {
                for i in 0..bank.batteries.len() - 1 {
                    if i > 0 {
                        for j in i..bank.batteries.len() {
                            dbg!(bank.batteries[i]);
                            dbg!(bank.batteries[j]);
                            pairs.push(bank.batteries[i] as u64 * 10 + bank.batteries[j] as u64);
                        }
                    }
                }
            };
            dbg!(&pairs);
            let max_pair = pairs.iter().max().unwrap().clone();
            dbg!(max_pair);
            max_pair
        };
        max_joltage += max_pair as u64;
    }
    Some(max_joltage)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
