use std::time::Instant;

use nom::{
    character::complete::{digit1, multispace1},
    combinator::map,
    sequence::tuple,
    bytes::complete::{tag, is_a},
    multi::separated_list1,
    IResult,
    Finish
};


#[derive(Debug)]
struct ClawMachine {
    button_a: Pos,
    button_b: Pos,
    prize: Pos,
}

#[derive(Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl std::ops::Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
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
    let machines = parse_input(input);
    let price = machines.into_iter()
        .filter_map(|machine| min_cost(&machine))
        .sum::<i64>();
    format!("{price}")
}


fn p2(input: &str) -> String {
    let machines = parse_input(input);
    let price = machines.into_iter()
        .map(|machine| correct_error(machine))
        .filter_map(|machine| min_cost(&machine))
        .sum::<i64>();
    format!("{price}")
}


fn correct_error(mut machine: ClawMachine) -> ClawMachine {
    machine.prize.x += 10000000000000;
    machine.prize.y += 10000000000000;

    machine
}


fn min_cost(machine: &ClawMachine) -> Option<i64> {
    // ax * a + bx * b = px
    // ay * a + by * b = py

    // b = py/by - ay*a/by
    // ax*a + bx*(py/by - ay*a/by) = px
    // ax*a + bx*py/by - ay*bx*a/by = px
    // a*(ax - ay*bx/by) = px - bx*py/by
    // a*(ax*by - ay*bx) = px*by - bx*py

    let ax = machine.button_a.x;
    let ay = machine.button_a.y;
    let bx = machine.button_b.x;
    let by = machine.button_b.y;
    let px = machine.prize.x;
    let py = machine.prize.y;

    let c1 = (px*by).checked_sub(bx*py)?;
    let c2 = (ax*by).checked_sub(ay*bx)?;
    let a = c1.checked_div(c2)?;
    let b = (px.checked_sub(ax*a))?.checked_div(bx)?;

    if a >= 0 && b >= 0 && (a*ax + b*bx == px) && (a*ay + b*by == py) {
        Some(a * 3 + b * 1)
    } else {
        None
    }
}


fn parse_input(input: &str) -> Vec<ClawMachine> {
    separated_list1(multispace1, parse_machine)
        (input)
        .finish()
        .unwrap()
        .1
}

fn parse_machine(input: &str) -> IResult<&str, ClawMachine> {
    map(
        tuple((
            parse_button,
            multispace1,
            parse_button,
            multispace1,
            parse_prize
        )), 
        |(button_a, _, button_b, _, prize)| {
            ClawMachine { button_a, button_b, prize }
        }
    )(input)
}


fn parse_button(input: &str) -> IResult<&str, Pos> {
    map(
        tuple((
            tag("Button "),
            is_a("AB"),
            tag(": X+"),
            parse_value,
            tag(", Y+"),
            parse_value,
        )),
        |(_, _, _, x, _, y)| Pos { x, y }
    )(input)
}


fn parse_prize(input: &str) -> IResult<&str, Pos> {
    map(
        tuple((
            tag("Prize: X="),
            parse_value,
            tag(", Y="),
            parse_value,
        )),
        |(_, x, _, y)| Pos { x, y }
    )(input)
}


fn parse_value(input: &str) -> IResult<&str, i64> {
    map(digit1, |s: &str| s.parse().unwrap())(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let machines = parse_input(EX);
        let price = machines.into_iter()
            .filter_map(|machine| min_cost(&machine))
            .sum::<i64>();
        assert_eq!(price, 480)
    }

}