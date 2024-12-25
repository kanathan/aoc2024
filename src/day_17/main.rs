use std::time::Instant;
use std::sync::LazyLock;

use itertools::Itertools;
use regex::Regex;

static REGISTER_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"Register \w: (?P<val>\d+)").unwrap()
});

#[derive(Clone, Debug)]
struct Program {
    reg_a: i64,
    _reg_b: i64,
    _reg_c: i64,
    program: Vec<i64>,
}

impl Program {
    fn new(reg_a: i64, _reg_b: i64, _reg_c: i64, program: Vec<i64>) -> Self {
        Self {
            reg_a,
            _reg_b,
            _reg_c,
            program,
        }
    }
}

enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<i64> for Opcode {
    fn from(val: i64) -> Self {
        match val {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            _ => panic!("Invalid instruction")
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
    let program = parse_input(input);
    let output = simple_run(program.reg_a).into_iter()
        .map(|v| format!("{v}"))
        .join(",");

    format!("{output:?}")
}


fn p2(input: &str) -> String {
    let program = parse_input(input);

    println!("{:?}", program.program);
    let a_min = 8_i64.pow(program.program.len() as u32 - 1);

    let results = find_solutions(&program.program, 0, a_min);

    format!("{}", results.into_iter().min().unwrap())
}


fn find_solutions(truth: &[i64], idx: usize, a_min: i64) -> Vec<i64> {
    let mut results = vec![];

    let sub_results =
        if idx < truth.len()-1 {
            find_solutions(truth, idx+1, a_min)
        } else {
            vec![a_min]
        };
    println!("Results for idx = {idx}");
    let pow_8 = 8_i64.pow(idx as u32);
    for a_min in sub_results {
        for a_delta in 0..8 {
            let mut output = vec![];
            let mut a = a_min + a_delta * pow_8;
            print!("{a}: ");
            for _ in 0..(truth.len()) {
                let (result_seg, new_a) = run_calc(a);
                a = new_a;
                output.push(result_seg);

            }
            println!("{output:?}");
            if output[idx] == truth[idx] {
                results.push(a_min + a_delta * pow_8);
            }
        }
    }

    results
}


fn run_calc(a: i64) -> (i64, i64) {
    let mut b = a % 8;
    b = b ^ 6;
    let c = a >> b;
    b = b ^ c;
    b = b ^ 4;
    let output = b % 8;
    return (output, a >> 3)
}


fn simple_run(mut a: i64) -> Vec<i64> {
    let mut output = vec![];
    loop {
        let mut b = a % 8;
        b = b ^ 6;
        let c = a >> b;
        b = b ^ c;
        b = b ^ 4;
        output.push(b % 8);
        a = a >> 3;
        if a == 0 { break }
    }
    
    output
}


fn parse_input(input: &str) -> Program {
    let mut line_iter = input.lines();

    let reg_a = REGISTER_RE.captures(
        line_iter.next().unwrap()
    ).unwrap()["val"].parse().unwrap();
    let reg_b = REGISTER_RE.captures(
        line_iter.next().unwrap()
    ).unwrap()["val"].parse().unwrap();
    let reg_c = REGISTER_RE.captures(
        line_iter.next().unwrap()
    ).unwrap()["val"].parse().unwrap();

    line_iter.next().unwrap();

    let program = line_iter.next().unwrap()
        .replace("Program: ", "")
        .split(",")
        .map(|s| s.parse::<_>().unwrap())
        .collect::<Vec<_>>();

    Program::new(reg_a, reg_b, reg_c, program)
}