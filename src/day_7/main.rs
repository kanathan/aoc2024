use std::time::Instant;

#[derive(Debug)]
struct Equation {
    answer: u64,
    inputs: Vec<u64>,
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
    let equations = parse_input(input);
    let sum = equations.into_iter()
        .filter(|eq| can_eval(eq.answer, eq.inputs.clone(), false))
        .map(|eq| eq.answer)
        .sum::<u64>();

    format!("{}", sum)
}


fn p2(input: &str) -> String {
    let equations = parse_input(input);
    let sum = equations.into_iter()
        .filter(|eq| can_eval(eq.answer, eq.inputs.clone(), true))
        .map(|eq| eq.answer)
        .sum::<u64>();

    format!("{}", sum)
}


fn can_eval(ans: u64, mut inputs: Vec<u64>, use_concat: bool) -> bool {
    let input = inputs.pop().unwrap();
    if inputs.is_empty() {
        return input == ans
    }
    let mult_branch = if ans % input == 0 {
        can_eval(ans/input, inputs.clone(), use_concat)
    } else {
        false
    };
    let add_branch = if ans >= input {
        can_eval(ans-input, inputs.clone(), use_concat)
    } else { 
        false
    };
    let concat_branch = if use_concat {
        let ans_str = format!("{ans}");
        let input_str = format!("{input}");
        if ans_str.len() > input_str.len() && ans_str.ends_with(&input_str) {
            let new_ans = ans_str[..(ans_str.len()-input_str.len())].parse::<u64>().unwrap();
            can_eval(new_ans, inputs.clone(), use_concat)
        } else {
            false
        }
    } else {
        false
    };

    return mult_branch || add_branch || concat_branch
}


fn parse_input(input: &str) -> Vec<Equation> {
    input.lines()
        .map(|line| {
            let (ans_str, inp_str) = line.split_once(':').unwrap();
            let answer = ans_str.parse::<u64>().unwrap();
            let inputs = inp_str.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            Equation {
                answer,
                inputs
            }
        })
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        assert_eq!(p1(EX), "3749");
    }

    #[test]
    fn test2() {
        let equations = parse_input(EX);
        for eq in equations {
            println!("{eq:?}");
            println!("{}", can_eval(eq.answer, eq.inputs, true));
        }
        assert_eq!(p2(EX), "11387");
    }

}

