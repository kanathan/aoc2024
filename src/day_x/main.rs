use std::time::Instant;

fn main() {
    let input = include_str!("input");

    let start = Instant::now();
    let parsed_input = parse_input(input);
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
    format!("{}", 0)
}


fn p2(input: &str) -> String {
    format!("{}", 0)
}


fn parse_input(input: &str) -> () {
    todo!()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {

    }

}