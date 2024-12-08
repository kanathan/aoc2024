use std::time::Instant;
use std::collections::{HashMap, HashSet};

use ndarray::Array2;
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
    let grid = parse_input(input);
    let anodes = get_antinodes_p1(&grid);
    format!("{}", anodes.len())
}


fn p2(input: &str) -> String {
    let grid = parse_input(input);
    let anodes = get_antinodes_p2(&grid);
    format!("{}", anodes.len())
}


fn get_antinodes_p1(grid: &Array2<char>) -> HashSet<(usize,usize)> {
    let mut freq_locs = HashMap::new();

    for (coords, c) in grid.indexed_iter() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                freq_locs.entry(c)
                    .or_insert(vec![])
                    .push(coords);
            },
            '.' => (),
            _ => unimplemented!("Didn't expect char {c}")
        }
    }

    let mut anodes = HashSet::new();

    for coord_list in freq_locs.values() {
        for (c1, c2) in coord_list.iter().tuple_combinations() {
            let c1 = (c1.0 as isize, c1.1 as isize);
            let c2 = (c2.0 as isize, c2.1 as isize);

            let node1 = (
                c1.0 - (c2.0 - c1.0),
                c1.1 - (c2.1 - c1.1)
            );
            let node2 = (
                c2.0 - (c1.0 - c2.0),
                c2.1 - (c1.1 - c2.1)
            );

            for node in [node1, node2] {
                if node.0 >= 0 && node.1 >= 0 {
                    let node = (node.0 as usize, node.1 as usize);
                    if grid.get(node).is_some() {
                        anodes.insert(node);
                    }
                }
            }
        }
    }

    anodes
}


fn get_antinodes_p2(grid: &Array2<char>) -> HashSet<(usize,usize)> {
    let mut freq_locs = HashMap::new();

    for (coords, c) in grid.indexed_iter() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' => {
                freq_locs.entry(c)
                    .or_insert(vec![])
                    .push(coords);
            },
            '.' => (),
            _ => unimplemented!("Didn't expect char {c}")
        }
    }

    let mut anodes = HashSet::new();

    for coord_list in freq_locs.values() {
        for (c1, c2) in coord_list.iter().tuple_combinations() {
            let c1 = (c1.0 as isize, c1.1 as isize);
            let c2 = (c2.0 as isize, c2.1 as isize);

            let row_delta = c2.0 - c1.0;
            let col_delta = c2.1 - c1.1;

            let mut i = 1;
            anodes.insert((c1.0 as usize, c1.1 as usize));
            let (mut anode1_valid, mut anode2_valid) = (true, true);
            while anode1_valid || anode2_valid {
                if anode1_valid {
                    let anode1 = (c1.0 - row_delta*i, c1.1 - col_delta*i);
                    if anode1.0 >= 0 && anode1.1 >= 0 {
                        let anode1 = (anode1.0 as usize, anode1.1 as usize);
                        if grid.get(anode1).is_some() {
                            anodes.insert(anode1);
                        } else {
                            anode1_valid = false;
                        }
                    } else {
                        anode1_valid = false;
                    }
                }
                
                if anode2_valid {
                    let anode2 = (c1.0 + row_delta*i, c1.1 + col_delta*i);
                    if anode2.0 >= 0 && anode2.1 >= 0 {
                        let anode2 = (anode2.0 as usize, anode2.1 as usize);
                        if grid.get(anode2).is_some() {
                            anodes.insert(anode2);
                        } else {
                            anode2_valid = false;
                        }
                    } else {
                        anode2_valid = false;
                    }
                }

                i += 1;
            }
        }
    }

    anodes
}


fn parse_input(input: &str) -> Array2<char> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().len();

    let data = input.lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<char>>();

    Array2::from_shape_vec((nrows,ncols), data).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let grid = parse_input(EX);
        let anodes = get_antinodes_p1(&grid);
        assert_eq!(anodes.len(), 14);

        let anodes = get_antinodes_p2(&grid);
        assert_eq!(anodes.len(), 34);
    }
}