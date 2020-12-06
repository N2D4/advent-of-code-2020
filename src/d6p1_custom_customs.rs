use crate::util;
use itertools::Itertools;

fn solve(inputs: impl AsRef<str>) -> usize {
    inputs
        .as_ref()
        .split("\n\n")
        .map(|s| s.chars().filter(|&c| c != '\n').unique().count())
        .sum()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            r"abc

a
b
c

ab
ac

a
a
a
a

b
"
        ),
        11
    );
}

#[test]
fn test() {
    let input = util::read_input("d6_custom_customs.txt");

    assert_eq!(solve(input), 6443);
}
