advent_of_code::solution!(11);

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn rows_cols_to_expand(grid: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let width = grid[0].len();
    let height = grid.len();

    let mut rows_to_expand = Vec::new();
    let mut cols_to_expand = Vec::new();

    for y in 0..height {
        if grid[y].iter().all(|c| *c == '.') {
            rows_to_expand.push(y);
        }
    }

    for x in 0..width {
        if grid.iter().all(|row| row[x] == '.') {
            cols_to_expand.push(x);
        }
    }

    (rows_to_expand, cols_to_expand)
}

fn parse_galaxies(grid: &Vec<Vec<char>>) -> Vec<Galaxy> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| Galaxy { x, y })
        })
        .collect()
}

struct Galaxy {
    x: usize,
    y: usize,
}

fn expand_with_factor(grid: &Vec<Vec<char>>, factor: u64) -> u64 {
    let (rows_to_expand, cols_to_expand) = rows_cols_to_expand(&grid);
    let galaxies = parse_galaxies(&grid);

    let mut distance_sum = 0;

    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            let galaxy1 = &galaxies[i];
            let galaxy2 = &galaxies[j];

            let min_x = galaxy1.x.min(galaxy2.x);
            let max_x = galaxy1.x.max(galaxy2.x);
            let min_y = galaxy1.y.min(galaxy2.y);
            let max_y = galaxy1.y.max(galaxy2.y);

            let mut dx = (max_x - min_x) as u64;
            let mut dy = (max_y - min_y) as u64;

            dx += (factor - 1)
                * (min_x..max_x)
                    .filter(|x| cols_to_expand.contains(x))
                    .count() as u64;
            dy += (factor - 1)
                * (min_y..max_y)
                    .filter(|y| rows_to_expand.contains(y))
                    .count() as u64;

            distance_sum += dx + dy;
        }
    }

    distance_sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_grid(input);
    let sum = expand_with_factor(&grid, 2);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    const ENLARGEMENT_FACTOR: u64 = 1000000;

    let grid = parse_grid(input);
    let sum = expand_with_factor(&grid, ENLARGEMENT_FACTOR);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(1030));
        assert_eq!(result, Some(8410));
    }
}
