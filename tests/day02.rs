fn parse() -> Vec<Vec<i32>> {
    let input = include_str!("res/02.txt");
    return input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
}

fn safe_part1(seq: &[i32]) -> bool {
    let first_delta = seq[0] - seq[1];
    if first_delta == 0 {
        return false;
    }
    let sign = first_delta.signum();
    seq.windows(2).all(|w| {
        let delta = w[0] - w[1];
        if sign != delta.signum() {
            return false;
        }
        let delta_abs = delta.abs();
        return delta_abs >= 1 && delta_abs <= 3;
    })
}

fn safe_part2(seq: &[i32]) -> bool {
    if safe_part1(seq) {
        return true;
    }
    // just brute force it :3
    return (0..seq.len())
        .map(|i| [&seq[..i], &seq[i + 1..]].concat())
        .any(|modified_report| safe_part1(&modified_report));
}

#[test]
fn part1() {
    let reports = parse();
    let ans = reports.iter().filter(|report| safe_part1(report)).count();
    println!("Day 2, Part 1: {ans}");
    assert_eq!(379, ans);
}

#[test]
fn part2() {
    let reports = parse();
    let ans = reports.iter().filter(|report| safe_part2(report)).count();
    println!("Day 2, Part 2: {ans}");
    assert_eq!(430, ans);
}
