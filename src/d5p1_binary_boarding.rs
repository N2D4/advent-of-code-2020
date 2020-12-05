use crate::util;

fn solve(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> i64 {
    inputs
        .into_iter()
        .map(|s| solve_line(s.as_ref()))
        .max()
        .unwrap()
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
fn test_example() {
    assert_eq!(solve_line("BFFFBBFRRR"), 567);
    assert_eq!(solve_line("FFFBBBFRRR"), 119);
    assert_eq!(solve_line("BBFFBBFRLL"), 820);
}

#[test]
fn test() {
    let input = util::read_input("d5_binary_boarding.txt");

    assert_eq!(solve(input.lines()), 989);
}
