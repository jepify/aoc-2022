use itertools::Itertools;

fn check(m: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> Option<()> {
    ((0..x).all(|i| m[y][i] < m[y][x])
        || ((x + 1)..m[y].len()).all(|i| m[y][i] < m[y][x])
        || (0..y).all(|i| m[i][x] < m[y][x])
        || ((y + 1)..m.len()).all(|i| m[i][x] < m[y][x]))
    .then_some(())
}

fn score(m: &Vec<Vec<u32>>, (x, y): (usize, usize)) -> usize {
    let mut pointer = x - 1;
    let vd_left = loop {
        if pointer == 0 {
            break x - pointer;
        }
        if m[y][pointer] >= m[y][x] {
            break x - pointer;
        }
        pointer -= 1;
    };
    pointer = x + 1;
    let vd_right = loop {
        if pointer == m[0].len() - 1 {
            break pointer - x;
        }
        if m[y][pointer] >= m[y][x] {
            break pointer - x;
        }
        pointer += 1;
    };
    pointer = y - 1;
    let vd_up = loop {
        if pointer == 0 {
            break y - pointer;
        }
        if m[pointer][x] >= m[y][x] {
            break y - pointer;
        }
        pointer -= 1;
    };
    pointer = y + 1;
    let vd_down = loop {
        if pointer == m.len() - 1 {
            break pointer - y;
        }

        if m[pointer][x] >= m[y][x] {
            break pointer - y;
        }
        pointer += 1;
    };
    // println!(
    //     "{},{} = {} -> {}{}{}{}",
    //     x, y, m[y][x], vd_left, vd_right, vd_up, vd_down
    // );
    vd_left * vd_right * vd_up * vd_down
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .split('\n')
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Some(
        (1..(grid.len() - 1))
            .cartesian_product(1..(grid[0].len() - 1))
            .filter_map(|(y, x)| check(&grid, (x, y)))
            .count()
            + grid.len() * 2
            + grid[0].len() * 2
            - 4,
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .split('\n')
        .map(|row| {
            row.chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (1..(grid.len() - 1))
        .cartesian_product(1..(grid[0].len() - 1))
        .map(|(y, x)| score(&grid, (x, y)))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
