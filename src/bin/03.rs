use itertools::Itertools;

advent_of_code::solution!(3);

fn neighbors<T: Copy>(
    grid: &Vec<Vec<T>>,
    [x, y]: [usize; 2],
) -> impl IntoIterator<Item = [usize; 2]> {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ]
    .into_iter()
    .map(|[dx, dy]| [x as i32 + dx, y as i32 + dy])
    .filter_map(|[x, y]| {
        if x >= 0 && y >= 0 && x < width && y < height {
            Some([x as usize, y as usize])
        } else {
            None
        }
    })
    .collect_vec()
}

fn neighbor_values<T: Copy>(grid: &Vec<Vec<T>>, [x, y]: [usize; 2]) -> impl IntoIterator<Item = T> {
    neighbors(grid, [x, y])
        .into_iter()
        .map(|[x, y]| grid[y][x])
        .collect_vec()
}

fn has_symbol_neighbor(grid: &Vec<Vec<char>>, [x, y]: [usize; 2]) -> bool {
    neighbor_values(grid, [x, y])
        .into_iter()
        .any(|char| char != '.' && !char.is_ascii_digit())
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let mut part_numbers = Vec::new();

    let mut current_number = Vec::new();
    let mut has_symbol = false;

    for y in 0..height {
        for x in 0..width {
            let char = grid[y][x];
            if char.is_ascii_digit() {
                current_number.push(char);
                if !has_symbol {
                    has_symbol = has_symbol_neighbor(&grid, [x, y]);
                }
            } else if !current_number.is_empty() {
                if has_symbol {
                    let number = current_number.iter().collect::<String>().parse().unwrap();
                    part_numbers.push(number);
                }
                current_number.clear();
                has_symbol = false;
            }
        }

        if !current_number.is_empty() {
            if has_symbol {
                let number = current_number.iter().collect::<String>().parse().unwrap();
                part_numbers.push(number);
            }
            current_number.clear();
            has_symbol = false;
        }
    }

    Some(part_numbers.iter().sum())
}

fn find_number(grid: &Vec<Vec<char>>, [x, y]: [usize; 2]) -> u32 {
    let line = &grid[y];
    let mut first_part = line[..x]
        .iter()
        .rev()
        .take_while(|char| char.is_ascii_digit())
        .collect_vec();
    first_part.reverse();
    let number = [
        first_part,
        line[x..]
            .iter()
            .take_while(|char| char.is_ascii_digit())
            .collect_vec(),
    ]
    .concat()
    .into_iter()
    .collect::<String>();
    number.parse().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let height = grid.len();
    let width = grid[0].len();

    let mut gear_ratio_sum = 0;

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == '*' {
                let neighbors = neighbors(&grid, [x, y]);

                let numbers = neighbors
                    .into_iter()
                    .filter(|[x, y]| grid[*y][*x].is_ascii_digit())
                    .map(|pos| find_number(&grid, pos))
                    .unique()
                    .collect_vec();
                if numbers.len() == 2 {
                    gear_ratio_sum += numbers[0] * numbers[1];
                }
            }
        }
    }

    Some(gear_ratio_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
