use std::collections::HashSet;

use crate::util;

fn find_marker(size: usize, data: &String) -> usize {
    for i in 0..data.len() - size - 1 {
        let chars = data[i..i + size].chars();
        let set: HashSet<char> = HashSet::from_iter(chars);
        if set.len() == size {
            let marker = i + size;
            return marker;
        }
    }
    return 0;
}

pub fn part1() {
    let binding = util::read_file_to_string("day_6.txt".to_string());
    let data = binding.first().unwrap();
    let index = find_marker(4, data);
    println!("Packet marker is at index {index}");
}

pub fn part2() {
    let binding = util::read_file_to_string("day_6.txt".to_string());
    let data = binding.first().unwrap();
    let index = find_marker(14, data);
    println!("Message marker is at index {index}");
}
