use crate::util;

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        return c as u32 - 96;
    }
    (c as u32 - 64) + 26
}

pub fn part1() {
    let data = util::read_file_to_string("day_3.txt".to_string());
    let mut sum = 0;
    for line in data {
        let split_index = line.len() / 2;
        let first = &line[0..split_index].to_string();
        let second = &line[split_index..line.len()].to_string();
        for item in first.chars() {
            if second.contains(item) {
                sum += get_priority(item);
                break;
            }
        }
    }
    println!("Sum of item priorities: {sum}");
}

pub fn part2() {
    let data = util::read_file_to_string("day_3.txt".to_string());
    let mut sum = 0;
    for i in (0..data.len() - 1).step_by(3) {
        let first = data[i].to_string();
        let second = data[i + 1].to_string();
        let third = data[i + 2].to_string();
        for item in first.chars() {
            if second.contains(item) && third.contains(item) {
                sum += get_priority(item);
                break;
            }
        }
    }
    println!("Sum of item priorities: {sum}");
}
