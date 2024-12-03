use std::time::Instant;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, char, anychar},
    combinator::{map_res, map},
    sequence::{separated_pair, delimited},
    multi::{many0, many_till},
    IResult
};

#[derive(Debug)]
enum Instr {
    Mul(u32, u32),
    Do,
    Dont,
}

fn main() {
    let input = include_str!("input");

    let start = Instant::now();
    let _parsed_input = parse_input(input);
    let parse_duration = start.elapsed().as_secs_f32();
    println!("Parsing took {parse_duration} secs");

    println!();

    let start = Instant::now();
    let p1_answer = p1(input);
    let p1_duration = start.elapsed().as_secs_f32();
    println!("P1: {p1_answer}");
    println!("Took {p1_duration} secs");

    println!();

    let start = Instant::now();
    let p2_answer = p2(input);
    let p2_duration = start.elapsed().as_secs_f32();
    println!("P2: {p2_answer}");
    println!("Took {p2_duration} secs");
}


fn p1(input: &str) -> String {
    let instrs = parse_input(input);
    let sum = get_mult_sum(&instrs);

    format!("{sum}")
}


fn p2(input: &str) -> String {
    let instrs = parse_input(input);
    let sum = get_mult_sum_p2(&instrs);

    format!("{sum}")
}


fn get_mult_sum(instrs: &[Instr]) -> u32 {
    instrs.into_iter()
        .filter_map(|instr| {
            match instr {
                Instr::Mul(a, b) => Some((a, b)),
                _ => None
            }
        })
        .map(|(a, b)| a*b)
        .sum::<u32>()
}


fn get_mult_sum_p2(instrs: &[Instr]) -> u32 {
    let mut enabled = true;
    let mut sum = 0;
    for instr in instrs.into_iter() {
        match instr {
            Instr::Mul(a, b) => if enabled {sum += a * b},
            Instr::Do => enabled = true,
            Instr::Dont => enabled = false,
        }
    }
    sum
}


fn parse_input(input: &str) -> Vec<Instr> {
    many0(parse_next_instr)
    (input)
        .unwrap().1
}


fn parse_next_instr(input: &str) -> IResult<&str, Instr> {
    map(
        many_till(
            anychar, 
            alt((parse_mul, parse_do, parse_dont))
        ),
        |(_, m)| m
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, Instr> {
    map(
        delimited(
            tag("mul("), 
            separated_pair(
                parse_number, 
                char(','), 
                parse_number
            ),
            char(')')
        ),
        |(a,b)| Instr::Mul(a, b)
    )(input)
}


fn parse_do(input: &str) -> IResult<&str, Instr> {
    map(tag("do()"), |_| Instr::Do)(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instr> {
    map(tag("don't()"), |_| Instr::Dont)(input)
}


fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let instrs = parse_input(EX);
        println!("{instrs:?}");
        assert_eq!(get_mult_sum(&instrs), 161);
    }

}