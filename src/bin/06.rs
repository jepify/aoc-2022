use std::{
    collections::{HashMap, HashSet},
    str::Chars,
};

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .chars()
        .into_iter()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .filter_map(|(i, (a, b, c, d))| {
            if a != b && a != c && a != d && b != c && b != d && c != d {
                Some(i as u32 + 4)
            } else {
                None
            }
        })
        .next()
}

pub fn part_two(input: &str) -> Option<usize> {
    for i in 0..input.len() - 14 {
        if input.get(i..i + 14).unwrap().chars().all_unique() {
            return Some(i + 14);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
