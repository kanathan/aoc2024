use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;

use ndarray::Array2;

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

    let score = grid.indexed_iter()
        .filter_map(|(pos, &val)| {
            if val == 0 {
                Some(pos)
            } else {
                None
            }
        })
        .map(|pos| trailhead_score(&grid, pos))
        .sum::<u32>();
    
    format!("{score}")
}


fn p2(input: &str) -> String {
    let grid = parse_input(input);

    let score = grid.indexed_iter()
        .filter_map(|(pos, &val)| {
            if val == 0 {
                Some(pos)
            } else {
                None
            }
        })
        .map(|pos| trailhead_rating(&grid, pos))
        .sum::<u32>();
    
    format!("{score}")
}


fn trailhead_score(grid: &Array2<u8>, starting_pos: (usize, usize)) -> u32 {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();

    visited.insert(starting_pos);
    to_visit.push_back(starting_pos);

    let mut score = 0;

    while let Some((row, col)) = to_visit.pop_front() {
        let prev_val = *grid.get((row, col)).unwrap();
        for (d_row, d_col) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let Ok(new_row) = usize::try_from(row as isize + d_row) else { continue };
            let Ok(new_col) = usize::try_from(col as isize + d_col) else { continue };
            let new_pos = (new_row, new_col);
            if visited.contains(&new_pos) {
                continue
            }

            let Some(&val) = grid.get(new_pos) else { continue };
            if val == prev_val + 1 {
                visited.insert(new_pos);
                if val == 9 {
                    score += 1
                } else {
                    to_visit.push_back(new_pos);
                }
            }
        }
    }

    score
}


fn trailhead_rating(grid: &Array2<u8>, starting_pos: (usize, usize)) -> u32 {
    let mut to_visit = VecDeque::new();

    to_visit.push_back(starting_pos);

    let mut score = 0;

    while let Some((row, col)) = to_visit.pop_front() {
        let prev_val = *grid.get((row, col)).unwrap();
        for (d_row, d_col) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let Ok(new_row) = usize::try_from(row as isize + d_row) else { continue };
            let Ok(new_col) = usize::try_from(col as isize + d_col) else { continue };
            let new_pos = (new_row, new_col);

            let Some(&val) = grid.get(new_pos) else { continue };
            if val == prev_val + 1 {
                if val == 9 {
                    score += 1
                } else {
                    to_visit.push_back(new_pos);
                }
            }
        }
    }

    score
}


fn parse_input(input: &str) -> Array2<u8> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().len();

    let data = input.lines()
        .map(|line| line.chars())
        .flatten()
        .map(|c| c as u8 - '0' as u8)
        .collect::<Vec<u8>>();

    Array2::from_shape_vec((nrows,ncols), data).unwrap()
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        assert_eq!(p1(EX), "36");
        assert_eq!(p2(EX), "81")
    }

}