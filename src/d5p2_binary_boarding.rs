use crate::util;
use std::collections::HashSet;

fn solve(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> i64 {
    let set: HashSet<i64> = inputs.into_iter().map(|s| solve_line(s.as_ref())).collect();
    *set.iter()
        .find(|&&i| !set.contains(&(i + 8)) && set.contains(&(i + 16)))
        .expect("no seat found!")
        + 8
}

fn solve_line(input: &str) -> i64 {
    i64::from_str_radix(
        &input
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1"),
        2,
    )
    .unwrap()
}

#[test]
fn test() {
    let input = util::read_input("d5_binary_boarding.txt");

    assert_eq!(solve(input.lines()), 548);
}
