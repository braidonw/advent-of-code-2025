advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Paper,
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Paper),
            _ => Err(()),
        }
    }
}

struct Map {
    cells: Vec<Vec<Cell>>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| Cell::try_from(c).ok())
                    .collect()
            })
            .collect();

        Self { cells }
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    fn cell_type(&self, i: usize, j: usize) -> Cell {
        self.cells[i][j]
    }

    fn remove(&mut self, i: usize, j: usize) {
        if self.cells[i][j] == Cell::Paper {
            self.cells[i][j] = Cell::Empty;
        }
    }

    fn can_access(&self, i: usize, j: usize) -> bool {
        let mut surrounding_paper = 0;

        // Check top left
        if i > 0 && j > 0 {
            if self.cells[i - 1][j - 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check top middle
        if i > 0 {
            if self.cells[i - 1][j] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check top right
        if i > 0 && j < self.cells[i].len() - 1 {
            if self.cells[i - 1][j + 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check middle left
        if j > 0 {
            if self.cells[i][j - 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check middle right
        if j < self.cells[i].len() - 1 {
            if self.cells[i][j + 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check bottom left
        if i < self.cells.len() - 1 && j > 0 {
            if self.cells[i + 1][j - 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check bottom middle
        if i < self.cells.len() - 1 {
            if self.cells[i + 1][j] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        // Check bottom right
        if i < self.cells.len() - 1 && j < self.cells[i].len() - 1 {
            if self.cells[i + 1][j + 1] == Cell::Paper {
                surrounding_paper += 1;
            }
        }

        surrounding_paper < 4
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;

    let map = Map::from_input(input);

    for i in 0..map.rows() {
        for j in 0..map.cols() {
            if map.cell_type(i, j) == Cell::Paper && map.can_access(i, j) {
                count += 1;
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    let mut map = Map::from_input(input);

    loop {
        let mut can_remove: Vec<(usize, usize)> = Vec::new();

        for i in 0..map.rows() {
            for j in 0..map.cols() {
                if map.cell_type(i, j) == Cell::Paper && map.can_access(i, j) {
                    can_remove.push((i, j));
                }
            }
        }

        if can_remove.is_empty() {
            break;
        }

        count += can_remove.len() as u64;

        for (i, j) in can_remove {
            map.remove(i, j);
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
