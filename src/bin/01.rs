pub fn part_one(input: &str) -> Option<u32> {
    // let a = input
    //     .split('\n')
    //     .map(str::parse::<u32>)
    //     .fold((Vec::new(), 0_u32), |(mut acc_list, acc), cur| match cur {
    //         Ok(cur) => (acc_list, acc + cur),
    //         Err(_) => {
    //             acc_list.push(acc);
    //             (acc_list, 0)
    //         }
    //     })
    //     .0
    //     .into_iter()
    //     .max();

    let input_split = input.split('\n').map(str::parse::<u32>);
    let mut max = 0_u32;
    let mut acc = 0_u32;
    for c in input_split {
        if let Ok(capacity) = c {
            acc += capacity;
        } else {
            max = max.max(acc);
            acc = 0;
        }
    }
    max = max.max(acc);
    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_split = input.split('\n').map(str::parse::<u32>);
    let mut max = vec![];
    let mut acc = 0_u32;
    for c in input_split {
        if let Ok(capacity) = c {
            acc += capacity;
        } else {
            max.push(acc);
            acc = 0;
        }
    }
    max.push(acc);
    max.sort();
    Some(max.into_iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
