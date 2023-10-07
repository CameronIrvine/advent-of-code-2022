use std::collections::BinaryHeap;

use crate::util;

pub fn part1() {
    let data = util::read_file_to_string("day_one.txt".to_string());
    let mut counts: Vec<i32> = vec![];
    let mut curr_count = 0;
    for line in data {
        if line.is_empty() {
            counts.push(curr_count);
            curr_count = 0;
            continue;
        }
        curr_count += line.parse::<i32>().unwrap();
    }
    let highest_count = counts.iter().max().unwrap();
    println!("Elf carrying the most calories: {highest_count}");
}

pub fn part2() {
    let data = util::read_file_to_string("day_one.txt".to_string());
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut curr_count = 0;
    for line in data {
        if line.is_empty() {
            heap.push(curr_count);
            curr_count = 0;
            continue;
        }
        curr_count += line.parse::<i32>().unwrap();
    }
    let first = heap.pop().unwrap();
    let second = heap.pop().unwrap();
    let third = heap.pop().unwrap();
    let total = first + second + third;
    println!("Top 3 elves carrying the most calories: {total}");
}
