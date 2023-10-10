use std::collections::HashMap;

use crate::util;

pub fn solution() {
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

    // part 2
    let fs_total_space = 70000000;
    let used_space = dirs.get("/").unwrap();
    let space_needed = (fs_total_space - used_space - 30000000).abs();
    let deleted_dir_size = dirs
        .iter()
        .filter(|x| x.1 >= &space_needed)
        .min_by(|a, b| a.1.cmp(b.1))
        .map(|(_k, v)| v.to_string())
        .unwrap();
    println!("Size of directory to be deleted is {deleted_dir_size}");
}
