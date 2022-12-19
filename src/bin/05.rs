use nom::{
    branch::alt,
    bytes::{
        complete::{is_a, is_not, tag},
        streaming::take_until,
    },
    character::{
        complete::{char, digit1, line_ending},
        is_alphabetic,
    },
    combinator::{map_res, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

fn to_option(input: &str) -> Result<Option<&str>, &str> {
    if input == "   " {
        Ok(None)
    } else {
        Ok(Some(input))
    }
}

fn supply_crate(input: &str) -> IResult<&str, Option<&str>> {
    map_res(
        alt((empty_crate, delimited(char('['), is_not("]"), char(']')))),
        to_option,
    )(input)
}

fn empty_crate(input: &str) -> IResult<&str, &str> {
    tag("   ")(input)
}

fn crate_row(input: &str) -> IResult<&str, Vec<Option<&str>>> {
    separated_list1(tag(" "), supply_crate)(input)
}

fn crate_rows(input: &str) -> IResult<&str, Vec<Vec<Option<&str>>>> {
    separated_list1(many0(line_ending), crate_row)(input)
}

fn crate_numbers(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(char(' '), separated_list1(tag("   "), digit1), char(' '))(input)
}

fn parse_crates(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let crates = separated_pair(crate_rows, line_ending, crate_numbers)(input).unwrap();

    // let stacks = Vec
    let crate_rows_numbers = crates.1 .1;
    let mut stacks = (0..crate_rows_numbers.len())
        .into_iter()
        .map(|x| vec![])
        .collect::<Vec<Vec<char>>>();
    for crate_row in crates.1 .0.into_iter().rev() {
        for (i, c) in crate_row.into_iter().enumerate() {
            if let Some(c) = c {
                stacks[i].push(c.chars().next().unwrap())
            }
        }
    }
    Ok((crates.0, stacks))
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    let (rest, (a, amount, b, from, c, to)) = tuple((
        tag("move "),
        digit1,
        tag(" from "),
        digit1,
        tag(" to "),
        digit1,
    ))(input)?;
    Ok((
        rest,
        Move {
            amount: str::parse(amount).unwrap(),
            from: str::parse::<usize>(from).unwrap() - 1,
            to: str::parse::<usize>(to).unwrap() - 1,
        },
    ))
}

fn parse_moves(input: &str) -> IResult<&str, Vec<Move>> {
    let rest = take_until("move")(input)?;
    separated_list1(line_ending, parse_move)(rest.0)
}

pub fn part_one(input: &str) -> Option<String> {
    let (rest, mut stacks) = parse_crates(input).unwrap();
    let (_, moves) = parse_moves(rest).unwrap();
    for m in moves {
        let from_stack = stacks.get_mut(m.from).unwrap();
        let remain = from_stack.split_off(from_stack.len() - m.amount);
        stacks.get_mut(m.to).unwrap().extend(remain.iter().rev())
    }
    let result = stacks.iter_mut().fold(String::new(), |mut acc, cur| {
        acc.push(cur.pop().unwrap());
        acc
    });

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    let (rest, mut stacks) = parse_crates(input).unwrap();
    let (_, moves) = parse_moves(rest).unwrap();
    for m in moves {
        let from_stack = stacks.get_mut(m.from).unwrap();
        let remain = from_stack.split_off(from_stack.len() - m.amount);
        stacks.get_mut(m.to).unwrap().extend(remain.iter())
    }
    let result = stacks.iter_mut().fold(String::new(), |mut acc, cur| {
        acc.push(cur.pop().unwrap());
        acc
    });

    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
