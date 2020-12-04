use crate::util;

fn solve(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    inputs
        .into_iter()
        .enumerate()
        .map(|(i, s)| s.as_ref().chars().nth((3 * i) % s.as_ref().len()))
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
        7
    );
}

#[test]
fn test() {
    let input = util::read_input("d3_toboggan_trajectory.txt");

    assert_eq!(solve(input.lines()), 232);
}
