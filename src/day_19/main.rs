use std::time::Instant;

type Pattern = String;


#[derive(Debug)]
struct TrieNode {
    children: [usize; 5],
    complete: bool
}

impl TrieNode {
    fn new() -> Self {
        Self { children: [0; 5], complete: false }
    }
}


#[derive(Debug)]
struct Trie {
    nodes: Vec<TrieNode>
}


impl Trie {
    fn new() -> Self {
        let mut nodes = Vec::with_capacity(1000);
        nodes.push(TrieNode::new());
        Self {
            nodes
        }
    }

    fn insert(&mut self, s: &str) {
        let mut cur_node_idx = 0;
        let color_idxs = s.chars()
            .map(map_color_to_idx);
        for color_idx in color_idxs {
            if self.nodes[cur_node_idx].children[color_idx] == 0 {
                self.nodes[cur_node_idx].children[color_idx] = self.nodes.len();
                self.nodes.push(TrieNode::new());
            }
            cur_node_idx = self.nodes[cur_node_idx].children[color_idx];
        }
        self.nodes[cur_node_idx].complete = true;
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
    let (available, designs) = parse_input(input);

    let trie = generate_trie(&available);

    let count = designs.iter()
        .filter(|design| find_possible_combos(&trie, design) > 0)
        .count();

    format!("{count}")
}


fn p2(input: &str) -> String {
    let (available, designs) = parse_input(input);

    let trie = generate_trie(&available);

    let sum = designs.iter()
        .map(|design| find_possible_combos(&trie, design))
        .sum::<usize>();

    format!("{sum}")
}


fn find_possible_combos(trie: &Trie, design: &str) -> usize {
    let mut possibilities = vec![0; design.len()+1];
    possibilities[0] = 1;

    let design_color_idxes = design.chars()
        .map(map_color_to_idx)
        .collect::<Vec<usize>>();

    for start_idx in 0..design.len() {
        if possibilities[start_idx] > 0 {
            let mut cur_trie_idx = 0;

            for end_idx in start_idx..design.len() {
                let color_idx = design_color_idxes[end_idx];
                cur_trie_idx = trie.nodes[cur_trie_idx].children[color_idx];

                if cur_trie_idx == 0 {
                    break
                }

                if trie.nodes[cur_trie_idx].complete {
                    possibilities[end_idx+1] += possibilities[start_idx];
                }
            }
        }
    }

    possibilities[design.len()]
}


fn generate_trie(patterns: &[Pattern]) -> Trie {
    let mut trie = Trie::new();

    for pattern in patterns {
        trie.insert(pattern);
    }

    trie
}



fn parse_input(input: &str) -> (Vec<Pattern>, Vec<String>) {
    let mut lines_iter = input.lines();

    let available_patterns = lines_iter.next().unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    lines_iter.next().unwrap();

    let designs = lines_iter
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    (available_patterns, designs)
}


fn map_color_to_idx(c: char) -> usize {
    match c {
        'w' => 0,
        'u' => 1,
        'b' => 2,
        'r' => 3,
        'g' => 4,
        _ => unimplemented!("Unknown color {c}")
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let (available, designs) = parse_input(EX);
        let trie = generate_trie(&available);

        let mut count = 0;
        for design in designs {
            let is_possible = design_possible(&trie, &design);
            println!("{design}: {}", is_possible);
            if is_possible { count += 1 };
        }
        assert_eq!(count, 6)
    }

}