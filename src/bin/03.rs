use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let a = input
        .trim_end()
        .split('\n')
        .into_iter()
        .map(|contents| {
            let (first, second) = contents.split_at(contents.len() / 2);
            let (first, second): (HashSet<char>, HashSet<char>) = (
                HashSet::from_iter(first.chars()),
                HashSet::from_iter(second.chars()),
            );
            let chr = first.intersection(&second).next().unwrap();
            if chr.is_uppercase() {
                *chr as u32 - 38
            } else {
                *chr as u32 - 96
            }
        })
        .sum::<u32>();
    Some(a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let a = input
        .trim_end()
        .split('\n')
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chr = chunk
                .map(|s| HashSet::from_iter(s.chars()))
                .fold(None, |acc: Option<HashSet<char>>, cur| match acc {
                    Some(inter) => Some(inter.intersection(&cur).into_iter().cloned().collect()),
                    None => Some(cur),
                })
                .map(|mut hs| hs.drain().next().unwrap())
                .unwrap();

            // .split('\n')
            // .map(|s| HashSet::from_iter(s.chars().)).fold(, |acc, cur| Some());
            if chr.is_uppercase() {
                chr as u32 - 38
            } else {
                chr as u32 - 96
            }
        })
        .sum::<u32>();
    Some(a)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
