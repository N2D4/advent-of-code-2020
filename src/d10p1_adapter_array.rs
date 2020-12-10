use std::iter::once;

use crate::util;
use itertools::Itertools;

fn solve(lines: Vec<i64>) -> usize {
    let difs: Vec<i64> = lines
        .iter()
        .copied()
        .chain(once(0))
        .sorted()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect();
    difs.iter().copied().filter(|&a| a == 1).count()
        * (difs.iter().copied().filter(|&a| a == 3).count() + 1)
}

#[test]
fn test_example_short() {
    assert_eq!(solve(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]), 35);
}

#[test]
fn test_example_long() {
    assert_eq!(
        solve(vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]),
        220
    );
}

#[test]
fn test() {
    let input = util::read_input("d10_adapter_array.txt");

    assert_eq!(
        solve(input.lines().map(|x| x.parse().unwrap()).collect()),
        2030
    );
}
