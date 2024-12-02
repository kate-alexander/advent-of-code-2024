use std::collections::HashMap;

fn parsed_input() -> (Vec<u32>, Vec<u32>) {
    let input = include_str!("res/01.txt");
    let (mut left, mut right) = (vec![], vec![]);
    for line in input.lines() {
        let (a, b) = line.split_once("   ").unwrap();
        left.push(a.parse().unwrap());
        right.push(b.parse().unwrap());
    }
    (left, right)
}

#[test]
fn part1() {
    let (mut left, mut right) = parsed_input();
    left.sort();
    right.sort();
    let ans: u32 = left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum();
    println!("Day 1, Part 1: {ans}");
    assert_eq!(2769675, ans);
}

#[test]
fn part2() {
    let (left, right) = parsed_input();
    let counts = right.iter().fold(HashMap::new(), |mut map, x| {
        map.entry(x).and_modify(|count| *count += 1).or_insert(1);
        map
    });
    let ans: u32 = left
        .iter()
        .map(|x| {
            let right_count = counts.get(x).unwrap_or(&0);
            x * right_count
        })
        .sum();
    println!("Day 1, Part 2: {ans}");
    assert_eq!(24643097, ans);
}
