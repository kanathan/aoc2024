use std::time::Instant;
use std::iter::zip;
use std::collections::HashMap;

use nom::{
    character::complete::{digit1, multispace1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
    Finish
};
use itertools::multiunzip;


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
    let (mut left_list, mut right_list) = parse_input(input);

    left_list.sort();
    right_list.sort();

    let tot_dist = zip(left_list, right_list)
        .map(|(l, r)| u32::abs_diff(l, r))
        .sum::<u32>();


    format!("{tot_dist}")
}


fn p2(input: &str) -> String {
    let (left_list, right_list) = parse_input(input);

    let mut right_counts: HashMap::<u32, u32> = HashMap::new();

    right_list.iter()
        .for_each(|val| {
            let count = right_counts.entry(*val).or_insert(0);
            *count += 1;
        });
    
    let score = left_list.iter()
        .map(|val| {
            let count = right_counts.get(val).unwrap_or(&0);
            (*val) * (*count)
        })
        .sum::<u32>();
    
    format!("{score}")
}


fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    multiunzip(input.lines()
        .map(parse_line)
    )
}


fn parse_line(input: &str) -> (u32, u32) {
    let parse = separated_pair(
        parse_number, 
        multispace1, 
        parse_number
    )(input);
    parse.finish().unwrap().1
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
        println!("{}", p2(EX))
    }

}