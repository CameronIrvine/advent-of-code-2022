use crate::util;

#[derive(Debug)]
struct Assignment {
    start: i32,
    end: i32,
}

fn get_assignment(range: String) -> Assignment {
    let index = range.chars().position(|c| c == '-').unwrap();
    let start: i32 = i32::from_str_radix(&range[0..index], 10).unwrap();
    let end = i32::from_str_radix(&range[index + 1..range.len()], 10).unwrap();
    Assignment { start, end }
}

pub fn part1() {
    let data = util::read_file_to_string("day_4.txt".to_string());
    let mut count = 0;
    for line in data {
        let ranges: Vec<String> = line.split(',').map(String::from).collect();
        let first = get_assignment(ranges[0].to_string());
        let second = get_assignment(ranges[1].to_string());
        if first.start == second.start && first.end == second.end {
            count += 1;
        } else if first.start <= second.start && second.end <= first.end {
            count += 1;
        } else if second.start <= first.start && first.end <= second.end {
            count += 1;
        }
    }
    println!("Count: {count}");
}

pub fn part2() {
    let data = util::read_file_to_string("day_4.txt".to_string());
    let mut count = 0;
    for line in data {
        let ranges: Vec<String> = line.split(',').map(String::from).collect();
        let first = get_assignment(ranges[0].to_string());
        let second = get_assignment(ranges[1].to_string());
        if !(first.end < second.start || second.end < first.start) {
            count += 1;
        }
    }
    println!("Count: {count}");
}
