use indexmap::IndexMap;

advent_of_code::solution!(15);

fn hash_value(step: &str) -> u32 {
    step.chars().fold(0, |acc, c| ((acc + c as u32) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(hash_value).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes = vec![IndexMap::new(); 256];

    input.split(',').for_each(|step| {
        if step.ends_with('-') {
            let label = &step[0..step.len() - 1];
            let box_id = hash_value(label);
            boxes[box_id as usize].shift_remove(&label);
        } else {
            let (label, focal_length) = step.split_once('=').unwrap();
            let box_id = hash_value(label);
            let focal_length = focal_length.parse::<u32>().unwrap();
            boxes[box_id as usize].insert(label, focal_length);
        }
    });

    let result = boxes
        .iter()
        .enumerate()
        .flat_map(|(i, lenses)| {
            lenses
                .values()
                .enumerate()
                .map(move |(j, focal_length)| (i as u32 + 1) * (j as u32 + 1) * focal_length)
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
