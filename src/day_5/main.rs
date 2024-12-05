use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
struct OrderRule {
    first: u32,
    last: u32,
}

enum UpdateStatus {
    Correct(u32),
    Incorrect(u32),
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


fn check_update(rules: &[OrderRule], update: &[u32]) -> UpdateStatus {
    let mut update = update.iter().copied().collect::<Vec<_>>();
    let mut update_hash = HashMap::new();
    for (idx, page) in update.iter().enumerate() {
        update_hash.insert(*page, idx);
    }

    let mut init_correct = true;

    'sort_loop: loop {
        for rule in rules.iter() {
            let Some(&first_idx) = update_hash.get(&rule.first) else { continue };
            let Some(&last_idx) = update_hash.get(&rule.last) else { continue };
            if first_idx > last_idx {
                init_correct = false;
                update.swap(first_idx, last_idx);
                update_hash.insert(rule.first, last_idx);
                update_hash.insert(rule.last, first_idx);
                continue 'sort_loop
            }
        }
        break
    }

    if init_correct {
        return UpdateStatus::Correct(update[update.len()/2])
    } else {
        return UpdateStatus::Incorrect(update[update.len()/2])
    }
}


fn p1(input: &str) -> String {
    let (rules, updates) = parse_input(input);
    let sum = updates.iter()
        .filter_map(|update| {
            match check_update(&rules, &update) {
                UpdateStatus::Correct(middle) => Some(middle),
                UpdateStatus::Incorrect(_) => None,        
            }
        })
        .sum::<u32>();
    format!("{sum}")
}


fn p2(input: &str) -> String {
    let (rules, updates) = parse_input(input);
    let sum = updates.iter()
        .filter_map(|update| {
            match check_update(&rules, &update) {
                UpdateStatus::Correct(_) => None,
                UpdateStatus::Incorrect(middle) => Some(middle),        
            }
        })
        .sum::<u32>();
    format!("{sum}")
}


fn parse_input(input: &str) -> (Vec<OrderRule>, Vec<Vec<u32>>) {
    let mut line_iter = input.lines();

    let mut order_rules = Vec::new();

    for line in line_iter.by_ref() {
        if line.is_empty() { break }
        let (first, last) = line.split_once('|').unwrap();
        order_rules.push(OrderRule {
            first: first.parse::<u32>().unwrap(),
            last: last.parse::<u32>().unwrap()
        });
    }

    let page_updates = line_iter
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    return (order_rules, page_updates)
}


#[cfg(test)]
mod test {
    use super::*;

    const EX: &str = include_str!("example");

    #[test]
    fn test1() {
        let (rules, updates) = parse_input(EX);
        let mut correct_sum = 0;
        let mut incorrect_sum = 0;
        for update in updates {
            match check_update(&rules, &update) {
                UpdateStatus::Correct(middle) => {
                    println!("{update:?} - Valid ({middle})");
                    correct_sum += middle;
                },
                UpdateStatus::Incorrect(middle) => {
                    println!("{update:?} - Invalid ({middle})");
                    incorrect_sum += middle;
                }
            }
        }
        assert_eq!(correct_sum, 143);
        assert_eq!(incorrect_sum, 123);
    }

}