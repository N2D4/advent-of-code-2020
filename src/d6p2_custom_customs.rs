use crate::util;
use std::collections::HashSet;

fn solve(inputs: impl AsRef<str>) -> usize {
    inputs
        .as_ref()
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.chars())
                .fold(('a'..='z').collect::<HashSet<char>>(), |set, a| {
                    a.filter(|c| set.contains(c)).collect()
                })
                .len()
        })
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
        6
    );
}

#[test]
fn test() {
    let input = util::read_input("d6_custom_customs.txt");

    assert_eq!(solve(input), 3232);
}
