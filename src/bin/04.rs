use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split('\n')
            .filter_map(|pair| {
                let pair = pair
                    .split(&['-', ','])
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .collect_vec();
                let ((a_start, a_end), (b_start, b_end)) = (
                    (pair.get(0).unwrap(), pair.get(1).unwrap()),
                    (pair.get(2).unwrap(), pair.get(3).unwrap()),
                );
                if (a_start >= b_start && a_end <= b_end) || (b_start >= a_start && b_end <= a_end)
                {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .trim_end()
            .split('\n')
            .filter_map(|pair| {
                let pair = pair
                    .split(&['-', ','])
                    .map(str::parse::<u32>)
                    .map(Result::unwrap)
                    .collect_vec();
                let ((a_start, a_end), (b_start, b_end)) = (
                    (pair.get(0).unwrap(), pair.get(1).unwrap()),
                    (pair.get(2).unwrap(), pair.get(3).unwrap()),
                );
                if (a_start >= b_start && a_start <= b_end)
                    || (b_start >= a_start && b_start <= a_end)
                {
                    Some(())
                } else {
                    None
                }
            })
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
