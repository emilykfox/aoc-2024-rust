use std::collections::{BinaryHeap, HashMap};

fn main() {
    let lines = aoc_2024::collect_lines("./inputs/day01.txt").unwrap();

    let mut queue_a = BinaryHeap::<u32>::new();
    let mut queue_b = BinaryHeap::<u32>::new();

    let mut hash_a = HashMap::<u32, u32>::new();
    let mut hash_b = HashMap::<u32, u32>::new();

    let re = regex::Regex::new(r"(\d\d\d\d\d)   (\d\d\d\d\d)").unwrap();
    for line in lines.iter() {
        let captures = re.captures(line).expect(line);
        let first: u32 = captures[1].parse().unwrap();
        let second: u32 = captures[2].parse().unwrap();
        queue_a.push(first);
        queue_b.push(second);
        *hash_a.entry(first).or_insert(0) += 1;
        *hash_b.entry(second).or_insert(0) += 1;
    }

    let mut suma = 0;

    while !queue_a.is_empty() {
        let first = queue_a.pop().unwrap();
        let second = queue_b.pop().unwrap();
        suma += first.abs_diff(second);
    }

    println!("Part A: {}", suma);

    let mut sumb = 0;
    for (key, value) in hash_a.iter() {
        sumb += *key * *value * *hash_b.get(key).unwrap_or(&0);
    }
    println!("Part B: {}", sumb);
}
