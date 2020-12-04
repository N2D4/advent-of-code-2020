use crate::util;

fn solve<T: AsRef<str>, I: IntoIterator<Item = T>>(inputs: I) -> usize {
    let vec: Vec<T> = inputs.into_iter().collect();
    solve_one(vec.iter(), 1, 1)
        * solve_one(vec.iter(), 3, 1)
        * solve_one(vec.iter(), 5, 1)
        * solve_one(vec.iter(), 7, 1)
        * solve_one(vec.iter(), 1, 2)
}

fn solve_one(inputs: impl IntoIterator<Item = impl AsRef<str>>, r: usize, d: usize) -> usize {
    inputs
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % d == 0)
        .map(|(i, s)| s.as_ref().chars().nth((r * i / d) % s.as_ref().len()))
        .filter(|c| c.unwrap() == '#')
        .count()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(&[
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]),
        336
    );
}

#[test]
fn test() {
    let input = util::read_input("d3_toboggan_trajectory.txt");

    assert_eq!(solve(input.lines()), 3952291680);
}
