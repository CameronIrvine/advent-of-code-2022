use std::collections::HashMap;

use crate::util;

pub fn part1() {
    let output = util::read_file_to_string("day_7.txt".to_string());
    let mut stack: Vec<String> = vec![];
    let mut dirs: HashMap<String, i32> = HashMap::new();
    for line in output {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        let split: Vec<String> = line.split(' ').map(String::from).collect();
        if line.starts_with("$ cd") {
            let dir = &split[2];
            match dir.as_str() {
                ".." => {
                    stack.pop();
                    continue;
                }
                _ => {
                    if stack.is_empty() {
                        stack.push(dir.to_owned());
                    } else {
                        stack.push(format!("{}/{}", stack.last().unwrap(), dir).replace("//", "/"));
                    }
                }
            }
        } else {
            let size = split[0].to_owned().parse::<i32>().unwrap();
            for path in stack.to_owned() {
                dirs.entry(path).and_modify(|s| *s += size).or_insert(size);
            }
        }
    }

    let mut sum = 0;
    for dir_size in dirs.values() {
        if dir_size.to_owned() <= 100000 {
            sum += dir_size;
        }
    }
    println!("Sum is {sum}");
}

// pub fn part2() {
//     let data = util::read_file_to_string("day_7.txt".to_string());
// }
