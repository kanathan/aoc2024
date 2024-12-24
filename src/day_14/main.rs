use std::{i32, time::Instant};

use ndarray::Array2;
use rustc_hash::FxHashSet;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, char, multispace1},
    combinator::{map, recognize, opt},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult
};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}


impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


#[derive(Debug, Clone)]
struct Robot {
    pos: Pos,
    vel: Pos
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
    let mut robots = parse_input(input);
    run((101, 103), &mut robots, 100);
    let sf = safety_factor((101, 103), &robots);
    format!("{sf}")
}


fn p2(input: &str) -> String {
    let robots = parse_input(input);
    let ticks = find_tree((101, 103), robots);
    format!("{ticks}")
}


fn run(grid_size: (u32, u32), robots: &mut Vec<Robot>, ticks: u32) {
    for _ in 0..ticks {
        for robot in robots.iter_mut() {
            robot.pos = robot.pos + robot.vel;
            robot.pos.x %= grid_size.0 as i32;
            robot.pos.y %= grid_size.1 as i32;
            if robot.pos.x < 0 {
                robot.pos.x += grid_size.0 as i32;
            }
            if robot.pos.y < 0 {
                robot.pos.y += grid_size.1 as i32;
            }
        }
    }
}


fn safety_factor(grid_size: (u32, u32), robots: &[Robot]) -> u32 {
    let mut quad_count = [0; 4];

    for robot in robots {
        if robot.pos.x < grid_size.0 as i32 / 2 {
            if robot.pos.y < grid_size.1 as i32 / 2 {
                quad_count[0] += 1;
            } else if robot.pos.y > grid_size.1 as i32 / 2 {
                quad_count[1] += 1;
            }
        } else if robot.pos.x > grid_size.0 as i32 / 2 {
            if robot.pos.y < grid_size.1 as i32 / 2 {
                quad_count[2] += 1;
            } else if robot.pos.y > grid_size.1 as i32 / 2 {
                quad_count[3] += 1;
            }
        }
    }

    quad_count.iter().product()
}


fn has_overlap(robots: &[Robot]) -> bool {
    let mut seen = FxHashSet::default();

    for robot in robots {
        if !seen.insert(robot.pos) {
            return true;
        }
    }

    false
}


fn find_tree(grid_size: (u32, u32), mut robots: Vec<Robot>) -> u32 {
    let mut ticks = 0;
    while has_overlap(&robots) {
        run(grid_size, &mut robots, 1);
        ticks += 1;
    }

    map_grid(grid_size, &robots);
    ticks
}


fn map_grid(grid_size: (u32, u32), robots: &[Robot]) {
    let mut grid = Array2::<u8>::zeros((grid_size.0 as usize, grid_size.1 as usize));

    for robot in robots {
        grid[(robot.pos.x as usize, robot.pos.y as usize)] += 1;
    }

    for y in 0..grid_size.1 {
        for x in 0..grid_size.0 {
            match grid[(x as usize, y as usize)] {
                0 => print!("."),
                x => print!("{x}"),
            }
        }
        println!();
    }
}


fn parse_input(input: &str) -> Vec<Robot> {
    separated_list1(multispace1, parse_robot)
    (input)
    .unwrap()
    .1
}


fn parse_robot(input: &str) -> IResult<&str, Robot> {
    map(
        tuple((
            preceded(tag("p="), parse_pos), 
            preceded(tag(" v="), parse_pos),
        )),
        |(pos, vel)| Robot { pos, vel }
    )(input)
}


fn parse_pos(input: &str) -> IResult<&str, Pos> {
    map(
        separated_pair(
            parse_value,
            char(','),
            parse_value
        ),
        |(x, y)| Pos { x, y }
    )(input)
}


fn parse_value(input: &str) -> IResult<&str, i32> {
    map(
        recognize(
            tuple((
                opt(char('-')),
                digit1
            ))
        ), 
        |s: &str| s.parse().unwrap()
    )(input)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let mut robots = parse_input(EX);
        run((11,7), &mut robots, 100);
        map_grid((11,7), &robots);
        assert_eq!(safety_factor((11,7), &robots), 12);
    }

}