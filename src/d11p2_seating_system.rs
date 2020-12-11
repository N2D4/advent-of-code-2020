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
                let neighbours = (-1..=1)
                    .cartesian_product(-1..=1)
                    .filter(|(a, b)| *a != 0 || *b != 0)
                    .map(|(a, b)| {
                        (1..)
                            .map(|d| (d * a, d * b))
                            .take_while(|(a, b)| {
                                x as isize + *a >= 0
                                    && y as isize + *b >= 0
                                    && x as isize + *a < last.len() as isize
                                    && y as isize + *b < line.len() as isize
                            })
                            .map(|(a, b)| last[offs(x, a)][offs(y, b)])
                            .find(|&c| c != '.')
                            .unwrap_or('.')
                    })
                    .filter(|&c| c == '#')
                    .count();
                new_line.push(match e {
                    'L' if neighbours == 0 => '#',
                    '#' if neighbours >= 5 => 'L',
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

fn offs(pos: usize, offset: isize) -> usize {
    (pos as isize + offset) as usize
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
        26
    );
}

#[test]
fn test() {
    let input = util::read_input("d11_seating_system.txt");

    assert_eq!(solve(input.lines()), 2138);
}
