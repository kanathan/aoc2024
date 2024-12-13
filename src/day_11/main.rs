use std::time::Instant;
use std::collections::HashMap;

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
    let input = parse_input(input);
    let result = blink(&input, 25);
    format!("{}", result)
}


fn p2(input: &str) -> String {
    let input = parse_input(input);
    let result = blink(&input, 75);
    format!("{}", result)
}



fn blink_once(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut blinked_stones = HashMap::with_capacity(stones.len());
    for (stone_val, count) in stones.into_iter() {
        if stone_val == 0 {
            *blinked_stones.entry(1).or_default() += count;
        } else {
            // Log10 of a number gives number of digits - 1
            let num_digits = u64::ilog10(stone_val) as u64 + 1;
            if num_digits % 2 == 0 {
                let mult = u64::pow(10, (num_digits/2) as u32);
                let val1 = stone_val / mult;
                let val2 = stone_val - (val1 * mult);
                *blinked_stones.entry(val1).or_default() += count;
                *blinked_stones.entry(val2).or_default() += count;
            } else {
                *blinked_stones.entry(stone_val * 2024).or_default() += count;
            }
        }
    }

    blinked_stones
}



fn blink(input: &[u64], depth: usize) -> u64 {
    let mut stones = HashMap::new();
    for val in input {
        *stones.entry(*val).or_default() += 1;
    }
    for _ in 0..depth {
        stones = blink_once(stones);
    }
    stones.values()
        .copied()
        .sum()
}


fn parse_input(input: &str) -> Vec<u64> {
    input.trim()
        .split_whitespace()
        .map(|s| s.to_string().parse::<u64>().unwrap())
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let input = parse_input(EX);
        let result = blink(&input, 6);
        assert_eq!(result, 22);
        let result = blink(&input, 25);
        assert_eq!(result, 55312);
    }

}