use std::time::Instant;


#[derive(Debug)]
struct Grid {
    data: Vec<Vec<char>>
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        Self {
            data
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<char> {
        if x < 0 || y < 0 {
            return None
        }
        let x = x as usize;
        let y = y as usize;
        let Some(line) = self.data.get(y) else {
            return None
        };
        let Some(&val) = line.get(x) else {
            return None
        };
        return Some(val)
    }

    fn x_len(&self) -> usize {
        if self.data.len() > 0 {
            self.data[0].len()
        } else {
            0
        }
    }

    fn y_len(&self) -> usize {
        self.data.len()
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
    let grid = parse_input(input);
    let count = search_grid_p1(&grid);
    format!("{count}")
}


fn p2(input: &str) -> String {
    let grid = parse_input(input);
    let count = search_grid_p2(&grid);
    format!("{count}")
}


fn search_grid_p1(grid: &Grid) -> u32 {
    let mut count = 0;
    for x in 0..grid.x_len() {
        for y in 0..grid.y_len() {
            count += xmas_count(grid, x, y);
        }
    }
    count
}


fn search_grid_p2(grid: &Grid) -> u32 {
    let mut count = 0;
    for x in 1..grid.x_len()-1 {
        for y in 1..grid.y_len()-1 {
            if has_x_mas(grid, x, y) {
                count += 1;
            }
        }
    }
    count
}


fn xmas_count(grid: &Grid, x: usize, y: usize) -> u32 {
    const DELTAS: [[(isize, isize); 4]; 8] = [
        [( 0, 0),( 1, 0),( 2, 0),( 3, 0)],
        [( 0, 0),(-1, 0),(-2, 0),(-3, 0)],
        [( 0, 0),( 0, 1),( 0, 2),( 0, 3)],
        [( 0, 0),( 0,-1),( 0,-2),( 0,-3)],
        [( 0, 0),( 1, 1),( 2, 2),( 3, 3)],
        [( 0, 0),(-1,-1),(-2,-2),(-3,-3)],
        [( 0, 0),( 1,-1),( 2,-2),( 3,-3)],
        [( 0, 0),(-1, 1),(-2, 2),(-3, 3)]
    ];

    const XMAS: [char; 4] = ['X','M','A','S'];

    let x = x as isize;
    let y = y as isize;

    let mut count = 0;
    for direction in DELTAS.into_iter() {
        let mut valid_word = true;
        for (to_find, (dx, dy)) in std::iter::zip(XMAS, direction) {
            let Some(c) = grid.get(x+dx, y+dy) else {
                valid_word = false;
                break
            };
            if c != to_find {
                valid_word = false;
                break
            }
        }
        if valid_word {
            count += 1;
        }
    }

    count
}


fn has_x_mas(grid: &Grid, x: usize, y: usize) -> bool {
    let x = x as isize;
    let y = y as isize;

    let Some(val) = grid.get(x, y) else { return false };
    if val != 'A' { return false };

    let Some(ul) = grid.get(x-1,y+1) else { return false };
    let Some(ll) = grid.get(x-1, y-1) else { return false };
    let Some(ur) = grid.get(x+1, y+1) else { return false };
    let Some(lr) = grid.get(x+1, y-1) else { return false };

    if !((ul == 'M' && lr == 'S') || (ul == 'S' && lr == 'M')) { return false };
    if !((ll == 'M' && ur == 'S') || (ll == 'S' && ur == 'M')) { return false };

    true
}


fn parse_input(input: &str) -> Grid {
    let data = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    Grid::new(data)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        assert_eq!(18, search_grid_p1(&parse_input(EX)));
        assert_eq!(9, search_grid_p2(&parse_input(EX)));
    }

}