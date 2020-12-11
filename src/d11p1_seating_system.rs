use std::cmp::{max, min};

use crate::util;
use itertools::Itertools;

fn solve(input: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    let mut last: Vec<Vec<char>> = input
        .into_iter()
        .map(|s| s.as_ref().chars().collect())
        .collect();
    loop {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for (x, line) in last.iter().cloned().enumerate() {
            let mut new_line: Vec<char> = Vec::new();
            for (y, &e) in line.iter().enumerate() {
                let neighbours = (max(x, 1) - 1..=min(x + 1, last.len() - 1))
                    .cartesian_product(max(y, 1) - 1..=min(y + 1, line.len() - 1))
                    .filter(|(a, b)| *a != x || *b != y)
                    .filter(|(a, b)| last[*a][*b] == '#')
                    .count();
                new_line.push(match e {
                    'L' if neighbours == 0 => '#',
                    '#' if neighbours >= 4 => 'L',
                    x => x,
                });
            }
            lines.push(new_line);
        }
        if last == lines {
            return lines
                .iter()
                .flat_map(|v| v.iter())
                .filter(|&&c| c == '#')
                .count();
        }
        last = lines;
    }
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]),
        37
    );
}

#[test]
fn test() {
    let input = util::read_input("d11_seating_system.txt");

    assert_eq!(solve(input.lines()), 2329);
}
