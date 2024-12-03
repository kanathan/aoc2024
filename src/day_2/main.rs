use std::time::Instant;

use nom::{
    character::complete::{digit1, char},
    combinator::map_res,
    multi::separated_list1,
    IResult,
    Finish
};
use itertools::Itertools;

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
    let reports = parse_input(input);

    let safe_report_count = reports.into_iter()
        .filter(|report| is_level_safe(report))
        .count();

    format!("{safe_report_count}")
}


fn p2(input: &str) -> String {
    let reports = parse_input(input);

    let safe_report_count = reports.into_iter()
        .map(|report| {
            if is_dampened_report_safe(&report) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();

    format!("{safe_report_count}")
}


fn is_level_safe(report: &[i32]) -> bool {
    let diff = report.into_iter()
        .tuple_windows()
        .map(|(&a, &b)| {
            b - a
        })
        .collect::<Vec<i32>>();

    diff.iter().all(|&v| v > 0 && v < 4) || diff.iter().all(|&v| v < 0 && v > -4)
}


fn is_dampened_report_safe(report: &[i32]) -> bool {
    if is_level_safe(&report) {
        return true
    }

    for idx in 0..report.len() {
        let sliced_report = [&report[..idx], &report[idx+1..]].concat();
        if is_level_safe(&sliced_report) {
            return true
        }
    }

    return false
}


fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(parse_line)
        .collect()
}


fn parse_line(input: &str) -> Vec<i32> {
    let parse = separated_list1(
        char(' '), 
        parse_number
    )(input);
    parse.finish().unwrap().1
}


fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        println!("{:?}", p1(EX));
        println!("{:?}", p2(EX));
    }

}