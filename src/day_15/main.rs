use std::{fmt::Write, time::Instant};

use ndarray::Array2;


#[derive(Clone, Copy)]
struct Pos {
    row: usize,
    col: usize,
}

struct Map {
    data: Array2<Object>,
    robot_pos: Pos,
}


impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for nrow in 0..(self.data.shape()[0]) {
            for ncol in 0..(self.data.shape()[1]) {
                if self.robot_pos.row == nrow && self.robot_pos.col == ncol {
                    f.write_char('@')?;
                } else {
                    match self.data.get((nrow, ncol)).unwrap() {
                        Object::Wall => f.write_char('#')?,
                        Object::Box => f.write_char('O')?,
                        Object::Empty => f.write_char('.')?,
                        Object::WideBoxLeft => f.write_char('[')?,
                        Object::WideBoxRight => f.write_char(']')?,
                    };
                }
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')?;
        Ok(())
    }
}


#[derive(Clone, Copy, Debug)]
enum Object {
    Wall,
    Box,
    Empty,
    WideBoxLeft,
    WideBoxRight,
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
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
    let (mut map, dirs) = parse_input(input);

    for dir in dirs.iter() {
        step(&mut map, *dir);
    }

    format!("{}", get_box_sum(&map.data))
}


fn p2(input: &str) -> String {
    let (mut map, dirs) = parse_input_wide(input);

    for dir in dirs.iter() {
        step(&mut map, *dir);
    }

    format!("{}", get_box_sum(&map.data))
}


fn step(map: &mut Map, dir: Dir) {
    let cur_pos = map.robot_pos;

    if let Some((new_pos, new_grid)) = try_move(map.data.clone(), cur_pos, dir) {
        map.data = new_grid;
        map.robot_pos = new_pos;
    }
}


fn try_move(mut grid: Array2<Object>, pos: Pos, dir: Dir) -> Option<(Pos, Array2<Object>)> {
    let this_obj = *grid.get((pos.row, pos.col))?;

    if matches!(this_obj, Object::Wall) {
        panic!("Attempted to move {this_obj:?}");
    }

    let mut new_pos = pos;
    match dir {
        Dir::Down => new_pos.row += 1,
        Dir::Up => new_pos.row = new_pos.row.checked_sub(1)?,
        Dir::Left => new_pos.col = new_pos.col.checked_sub(1)?,
        Dir::Right => new_pos.col += 1,
    };
    let new_pos_obj = *grid.get((new_pos.row, new_pos.col))?;

    match new_pos_obj {
        Object::Wall => return None,
        Object::Box => {
            let (_, new_grid) = try_move(grid, new_pos, dir)?;
            grid = new_grid;
        },
        Object::Empty => (),
        Object::WideBoxLeft => {
            let (_, new_grid) = try_move(grid, new_pos, dir)?;
            if !matches!(dir, Dir::Right) {
                let new_pos_box = Pos { row: new_pos.row, col: new_pos.col + 1 };
                let (_, new_grid) = try_move(new_grid, new_pos_box, dir)?;
                grid = new_grid;
            } else {
                grid = new_grid;
            }
        },
        Object::WideBoxRight => {
            let (_, new_grid) = try_move(grid, new_pos, dir)?;
            if !matches!(dir, Dir::Left) {
                let new_pos_box = Pos { row: new_pos.row, col: new_pos.col.checked_sub(1)? };
                let (_, new_grid) = try_move(new_grid, new_pos_box, dir)?;
                grid = new_grid;
            } else {
                grid = new_grid;
            }
        }
    }

    *grid.get_mut((new_pos.row, new_pos.col)).unwrap() = this_obj;
    *grid.get_mut((pos.row, pos.col)).unwrap() = Object::Empty;

    Some((new_pos, grid))
}


fn get_box_sum(grid: &Array2<Object>) -> u64 {
    grid.indexed_iter()
        .filter(|(_, &obj)| matches!(obj, Object::Box) || matches!(obj, Object::WideBoxLeft))
        .map(|((row, col), _)| row as u64 * 100 + col as u64)
        .sum() 
}


fn parse_input_wide(input: &str) -> (Map, Vec<Dir>) {
    let mut lines_iter = input.lines();
    let mut map_data = vec![];
    let mut robot_pos = None;

    let ncols = input.lines().next().unwrap().len()*2;
    let mut nrows = 0;

    while let Some(line) = lines_iter.next() {
        if line.is_empty() { break }
        nrows += 1;

        for (icol, c) in line.chars().enumerate() {
            match c {
                '#' => { map_data.push(Object::Wall); map_data.push(Object::Wall); },
                '.' => { map_data.push(Object::Empty); map_data.push(Object::Empty); },
                'O' => { map_data.push(Object::WideBoxLeft); map_data.push(Object::WideBoxRight); },
                '@' => {
                    if robot_pos.is_some() { panic!("Already found robot") }
                    robot_pos = Some(Pos { row: nrows-1, col: icol*2 } );
                    map_data.push(Object::Empty); map_data.push(Object::Empty);
                },
                _ => unimplemented!("Unknown map object: {c}")
            }
        }
    }

    let map_data: Array2<Object> = Array2::from_shape_vec((nrows, ncols), map_data).unwrap();

    let dirs = lines_iter
        .map(|line| line.chars())
        .flatten()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => unimplemented!("Unknown dir: {c}")
        })
        .collect::<Vec<Dir>>();
    
    (
        Map { data: map_data, robot_pos: robot_pos.unwrap() },
        dirs
    )
}


fn parse_input(input: &str) -> (Map, Vec<Dir>) {
    let mut lines_iter = input.lines();
    let mut map_data = vec![];
    let mut robot_pos = None;

    let ncols = input.lines().next().unwrap().len();
    let mut nrows = 0;

    while let Some(line) = lines_iter.next() {
        if line.is_empty() { break }
        nrows += 1;

        for (icol, c) in line.chars().enumerate() {
            map_data.push(
                match c {
                    '#' => Object::Wall,
                    '.' => Object::Empty,
                    'O' => Object::Box,
                    '@' => {
                        if robot_pos.is_some() { panic!("Already found robot") }
                        robot_pos = Some(Pos { row: nrows-1, col: icol } );
                        Object::Empty
                    },
                    _ => unimplemented!("Unknown map object: {c}")
                }
            )
        }
    }

    let map_data: Array2<Object> = Array2::from_shape_vec((nrows, ncols), map_data).unwrap();

    let dirs = lines_iter
        .map(|line| line.chars())
        .flatten()
        .map(|c| match c {
            '<' => Dir::Left,
            '>' => Dir::Right,
            '^' => Dir::Up,
            'v' => Dir::Down,
            _ => unimplemented!("Unknown dir: {c}")
        })
        .collect::<Vec<Dir>>();
    
    (
        Map { data: map_data, robot_pos: robot_pos.unwrap() },
        dirs
    )
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");
    const EX2: &str = include_str!("example2");
    const EX3: &str = include_str!("example3");

    #[test]
    fn test1() {
        let (mut map, dirs) = parse_input(EX);

        println!("Initial");
        println!("{map}");
        println!();

        for dir in dirs {
            step(&mut map, dir);
        }

        println!("{map}");
        println!();

        assert_eq!(get_box_sum(&map.data), 10092);
    }

    #[test]
    fn test2() {
        let (mut map, dirs) = parse_input(EX2);

        println!("Initial");
        println!("{map}");
        println!();

        for (idx, &dir) in dirs.iter().enumerate() {
            step(&mut map, dir);
            println!("Step {idx}: Move {dir:?}");
            println!("{map}");
            println!();
        }

        assert_eq!(get_box_sum(&map.data), 2028);
    }

    #[test]
    fn test3() {
        let (mut map, dirs) = parse_input_wide(EX);

        println!("Initial");
        println!("{map}");
        println!();

        for (_idx, &dir) in dirs.iter().enumerate() {
            step(&mut map, dir);
        }

        println!("{map}");
        println!();

        assert_eq!(get_box_sum(&map.data), 9021);
    }

    #[test]
    fn test4() {
        let (mut map, dirs) = parse_input_wide(EX3);

        println!("Initial");
        println!("{map}");
        println!();

        for (idx, &dir) in dirs.iter().enumerate() {
            step(&mut map, dir);
            println!("Step {idx}: Move {dir:?}");
            println!("{map}");
            println!();
        }

        // assert_eq!(get_box_sum(&map.data), 9021);
    }

}