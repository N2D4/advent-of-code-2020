use crate::util;
use itertools::Itertools;
use std::{iter::repeat_with, ops::RangeInclusive};

fn solve<'a>(
    rules: impl IntoIterator<Item = (&'a str, impl IntoIterator<Item = RangeInclusive<usize>>)>,
    nearby: impl IntoIterator<Item = impl IntoIterator<Item = usize>>,
) -> usize {
    let mut applicable_rules: Vec<Vec<&'a str>> = repeat_with(|| Vec::new()).take(1000).collect();
    for rule in rules {
        for range in rule.1 {
            for i in range {
                applicable_rules[i].push(rule.0);
            }
        }
    }

    let mut res = 0;
    for ticket in nearby {
        for field in ticket {
            if applicable_rules[field].len() == 0 {
                res += field;
            }
        }
    }
    res
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            vec![
                ("class", vec![1..=3, 5..=7]),
                ("row", vec![6..=11, 33..=44]),
                ("seat", vec![13..=40, 45..=50])
            ],
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        ),
        71
    );
}

#[test]
fn test() {
    let input = util::read_input("d16_ticket_translation.txt");
    let (ticket_rules, rest) = input.split("\nyour ticket:\n").next_tuple().unwrap();
    let (_, nearby_tickets) = rest.split("\nnearby tickets:\n").next_tuple().unwrap();

    assert_eq!(
        solve(
            ticket_rules.lines().map(|l| l
                .split(": ")
                .next_tuple()
                .map(|(name, rule)| (
                    name,
                    rule.split(" or ").map(|s| s
                        .split("-")
                        .map(|x| x.parse().unwrap())
                        .next_tuple()
                        .map(|(a, b)| a..=b)
                        .unwrap())
                ))
                .unwrap()),
            nearby_tickets
                .lines()
                .map(|x| x.split(",").map(|x| x.parse().unwrap()))
        ),
        29878
    );
}
