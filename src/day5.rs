use std::collections::{HashMap, VecDeque};

use crate::util;

pub fn part1() {
    let data = util::read_file_to_string("day_5.txt".to_string());
    let mut stacks: HashMap<i32, VecDeque<char>> = HashMap::new();
    for line in data {
        if line.contains('[') {
            // this is a stack
            let split: Vec<char> = line.chars().collect();
            let mut curr_stack = 0;
            for i in (0..split.len() - 1).step_by(4) {
                curr_stack += 1;
                if split[i] != '[' {
                    continue;
                }
                stacks
                    .entry(curr_stack)
                    .or_insert(VecDeque::new())
                    .push_back(split[i + 1]);
            }
        }
        if line.contains("move") {
            // movement instruction
            let split: Vec<String> = line.split(' ').map(String::from).collect();
            let amount = split[1].parse::<i32>().unwrap();
            let source = split[3].parse::<i32>().unwrap();
            let target = split[5].parse::<i32>().unwrap();
            let mut source_stack = stacks.get(&source).unwrap().to_owned();
            let mut target_stack = stacks.get(&target).unwrap().to_owned();
            for _ in 0..amount {
                let cr = source_stack.pop_front().unwrap();
                target_stack.push_front(cr.clone());
            }
            stacks.insert(source, source_stack);
            stacks.insert(target, target_stack);
        }
    }

    let num_stacks = stacks.keys().len() as i32;
    let mut crates_at_top = String::new();
    for i in 1..num_stacks + 1 {
        crates_at_top += &stacks.get(&i).unwrap().front().unwrap().to_string();
    }
    println!("Crates at the top of each stack: {crates_at_top}");
}

// pub fn part2() {
//     let data = util::read_file_to_string("day_4.txt".to_string());
// }
