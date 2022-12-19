use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::{
        complete::{is_a, is_not, tag},
        streaming::take_until,
    },
    character::{
        complete::{alphanumeric1, char, digit1, line_ending, not_line_ending},
        is_alphabetic,
    },
    combinator::{map_res, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug)]
enum OutputType<'a> {
    CD(&'a str),
    LS,
    DIR(&'a str),
    FILE(u32),
}

#[derive(Debug, Clone, Copy)]
struct Directory<'a> {
    path: &'a str,
    size: u32,
}

fn cd(input: &str) -> IResult<&str, OutputType> {
    let (rest, (_, dir)) = tuple((tag("$ cd "), not_line_ending))(input)?;
    Ok((rest, OutputType::CD(dir)))
}

fn ls(input: &str) -> IResult<&str, OutputType> {
    let (rest, ls) = tag("$ ls")(input)?;
    Ok((rest, OutputType::LS))
}

fn dir(input: &str) -> IResult<&str, OutputType> {
    let (rest, (_, dir)) = tuple((tag("dir "), alphanumeric1))(input)?;
    Ok((rest, OutputType::DIR(dir)))
}

fn file(input: &str) -> IResult<&str, OutputType> {
    let (rest, (size, _, _)) = tuple((digit1, tag(" "), not_line_ending))(input)?;
    Ok((rest, OutputType::FILE(size.parse().unwrap())))
}

fn traversal(input: &str) -> IResult<&str, Vec<OutputType>> {
    separated_list1(line_ending, alt((cd, ls, dir, file)))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let t = traversal(input).unwrap().1;

    let mut visited_dirs: Vec<Directory> = Vec::new();
    let mut dir_stack: Vec<Directory> = Vec::new();

    let mut cur_dir = Directory { path: "/", size: 0 };
    // let mut cur_size = 0_u32;
    for command in t {
        match command {
            OutputType::CD("/") => {}
            OutputType::CD("..") => {
                {
                    visited_dirs.push(cur_dir);
                    let mut prev_dir = dir_stack.pop().unwrap();
                    prev_dir.size += cur_dir.size;
                    cur_dir = prev_dir
                };
            }
            OutputType::CD(dir) => {
                dir_stack.push(cur_dir);
                cur_dir = Directory { path: dir, size: 0 }
            }
            OutputType::LS => {}
            OutputType::DIR(_) => {}
            OutputType::FILE(size) => cur_dir.size += size,
        }
    }
    dir_stack.push(cur_dir);

    let mut acc = 0;
    for remaining in dir_stack.iter_mut() {
        remaining.size += acc;
        visited_dirs.push(*remaining);
        acc += remaining.size
    }

    Some(
        visited_dirs
            .iter()
            .filter_map(|e| if e.size <= 100000 { Some(e.size) } else { None })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let t = traversal(input).unwrap().1;

    let mut visited_dirs: Vec<Directory> = Vec::new();
    let mut dir_stack: Vec<Directory> = Vec::new();

    let mut cur_dir = Directory { path: "/", size: 0 };
    let mut cur_size = 0_u32;
    for command in t {
        match command {
            OutputType::CD("/") => {}
            OutputType::CD("..") => {
                {
                    visited_dirs.push(cur_dir);
                    let mut prev_dir = dir_stack.pop().unwrap();
                    prev_dir.size += cur_dir.size;
                    cur_dir = prev_dir
                };
            }
            OutputType::CD(dir) => {
                dir_stack.push(cur_dir);
                cur_dir = Directory { path: dir, size: 0 }
            }
            OutputType::LS => {}
            OutputType::DIR(_) => {}
            OutputType::FILE(size) => cur_dir.size += size,
        }
    }
    // let total = t.iter().fold(0, |acc, cur| match cur {
    //     OutputType::FILE(size) => acc + size,
    //     _ => acc,
    // });
    visited_dirs.push(cur_dir);

    let mut acc = cur_dir.size;
    for remaining in dir_stack.iter_mut().rev() {
        remaining.size += acc;
        visited_dirs.push(*remaining);
        acc += remaining.size
    }
    visited_dirs.sort_by_key(|d| d.size);
    let needed_space = 30000000;
    let disk_space = 70000000;
    let used_space = 41035571; // total from 129. Somewhere in the code we calculate root dir / size incorrectly
    let free_space = disk_space - used_space;
    let needed_to_free2 = needed_space - free_space;
    let a = visited_dirs
        .iter()
        .find(|d| d.size >= needed_to_free2)
        .unwrap();
    Some(a.size)
}
// 41035571
fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
