use crate::util;
use std::collections::HashSet;

fn solve(inputs: &[i64]) -> i64 {
    let set: HashSet<i64> = inputs.iter().copied().collect();
    let x = set.iter()
        .find(|&a| set.contains(&(2020 - a)))
        .expect("no such pair!");
    return x * (2020 - x);
}

#[test]
fn test_d1p1_example() {
    assert_eq!(solve(&[1721, 979, 366, 299, 675, 1456]), 514579);
}

#[test]
fn test_d1p1() {
    let vec: Vec<i64> = util::read_input("d1_report_repair.txt").lines()
        .map(|x| x.parse().expect("value not an i64!"))
        .collect();

    assert_eq!(solve(&vec), 996996);
}
