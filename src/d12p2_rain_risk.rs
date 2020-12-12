use crate::util;

fn solve(input: impl IntoIterator<Item = impl AsRef<str>>) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut wp: (i32, i32) = (10, -1);
    for line in input {
        let spl = (
            line.as_ref().chars().next().unwrap(),
            &line.as_ref()[1..].parse().unwrap(),
        );
        match spl {
            ('L', 90) | ('R', 270) => wp = (wp.1, -wp.0),
            ('L', 180) | ('R', 180) => wp = (-wp.0, -wp.1),
            ('L', 270) | ('R', 90) => wp = (-wp.1, wp.0),
            ('W', x) => wp.0 -= x,
            ('E', x) => wp.0 += x,
            ('N', x) => wp.1 -= x,
            ('S', x) => wp.1 += x,
            ('F', x) => pos = (pos.0 + x * wp.0, pos.1 + x * wp.1),
            _ => panic!(),
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[test]
fn test_example() {
    assert_eq!(solve(vec!["F10", "N3", "F7", "R90", "F11"]), 286);
}

#[test]
fn test() {
    let input = util::read_input("d12_rain_risk.txt");

    assert_eq!(solve(input.lines()), 18107);
}
