use crate::util;
use itertools::Itertools;
use std::{collections::HashSet, iter::repeat_with, ops::RangeInclusive};

fn solve<'a>(
    rules: impl IntoIterator<Item = (&'a str, impl IntoIterator<Item = RangeInclusive<usize>>)>,
    nearby: impl IntoIterator<Item = impl IntoIterator<Item = usize>>,
) -> Vec<&'a str> {
    let mut applicable_rules: Vec<HashSet<&'a str>> =
        repeat_with(|| HashSet::new()).take(1000).collect();
    let mut all_rules: HashSet<&'a str> = HashSet::new();
    for rule in rules {
        all_rules.insert(rule.0);
        for range in rule.1 {
            for i in range {
                applicable_rules[i].insert(rule.0);
            }
        }
    }

    let filtered = nearby
        .into_iter()
        .map(|t| t.into_iter().collect_vec())
        .filter(|t| !t.iter().any(|&x| applicable_rules[x].len() == 0));

    let mut possible: Vec<HashSet<&'a str>> = repeat_with(|| all_rules.clone())
        .take(all_rules.len())
        .collect();
    for ticket in filtered {
        for i in 0..all_rules.len() {
            possible[i].retain(|a| applicable_rules[ticket[i]].contains(a));
        }
    }

    // find unique perfect cardinal bipartite matching
    let mut already_out: HashSet<&'a str> = HashSet::new();
    for _ in 0..=all_rules.len() {
        for j in 0..all_rules.len() {
            if possible[j].len() == 1 {
                already_out.insert(possible[j].iter().next().unwrap());
            } else {
                possible[j].retain(|x| !already_out.contains(x));
            }
        }
    }

    possible
        .iter()
        .map(|set| set.iter().copied().next().unwrap())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            vec![
                ("class", vec![0..=1, 4..=19]),
                ("row", vec![0..=5, 8..=19]),
                ("seat", vec![0..=13, 16..=19])
            ],
            vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9],]
        ),
        vec!["row", "class", "seat"]
    );
}

#[test]
fn test() {
    let input = util::read_input("d16_ticket_translation.txt");
    let (ticket_rules, rest) = input.split("\nyour ticket:\n").next_tuple().unwrap();
    let (your_ticket, nearby_tickets) = rest.split("\nnearby tickets:\n").next_tuple().unwrap();

    assert_eq!(
        solve(
            ticket_rules.lines().map(|l| {
                l.split(": ")
                    .next_tuple()
                    .map(|(name, rule)| {
                        (
                            name,
                            rule.split(" or ").map(|s| {
                                s.split("-")
                                    .map(|x| x.parse().unwrap())
                                    .next_tuple()
                                    .map(|(a, b)| a..=b)
                                    .unwrap()
                            }),
                        )
                    })
                    .unwrap()
            }),
            nearby_tickets
                .lines()
                .map(|x| x.split(",").map(|x| x.parse().unwrap())),
        )
        .iter()
        .enumerate()
        .filter(|(_, s)| s.starts_with("departure"))
        .map(|(i, _)| your_ticket
            .split(",")
            .nth(i)
            .unwrap()
            .parse::<usize>()
            .unwrap())
        .fold(1, |a, b| a * b),
        855438643439
    );
}
