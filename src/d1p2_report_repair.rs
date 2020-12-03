use crate::util;
use std::collections::HashSet;

fn solve(inputs: &[i64]) -> i64 {
    let set: HashSet<i64> = inputs.iter().copied().collect();
    let x = set.iter()
        .flat_map(|&a| set.iter().map(move |&b| [a, b]))
        .find(|&[a, b]| set.contains(&(2020 - a - b)))
        .expect("no such triplet!");
    return x[0] * x[1] * (2020 - x[0] - x[1]);
}

#[test]
fn test_d1p2_example() {
    assert_eq!(solve(&[1721, 979, 366, 299, 675, 1456]), 241861950);
}

#[test]
fn test_d1p2() {
    let vec: Vec<i64> = util::read_input("d1_report_repair.txt").lines()
        .map(|x| x.parse().expect("value not an i64!"))
        .collect();

    assert_eq!(solve(&vec), 9210402);
}
