use crate::util;

fn solve(input: impl IntoIterator<Item = impl AsRef<str>>) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = 0;
    for line in input {
        let mut spl = (
            line.as_ref().chars().next().unwrap(),
            &line.as_ref()[1..].parse().unwrap(),
        );
        if spl.0 == 'F' {
            spl.0 = ['E', 'S', 'W', 'N'][dir as usize];
        }
        match spl.0 {
            'L' => dir = (dir + 3 * spl.1 / 90) % 4,
            'R' => dir = (dir + 1 * spl.1 / 90) % 4,
            'W' => pos.0 -= spl.1,
            'E' => pos.0 += spl.1,
            'N' => pos.1 -= spl.1,
            'S' => pos.1 += spl.1,
            _ => panic!(),
        }
    }
    pos.0.abs() + pos.1.abs()
}

#[test]
fn test_example1() {
    assert_eq!(solve(vec!["F10", "N3", "F7", "R90", "F11"]), 25);
}

#[test]
fn test_example2() {
    assert_eq!(solve(vec!["F10", "L90", "F3", "L270", "F7", "S11"]), 25);
}

#[test]
fn test() {
    let input = util::read_input("d12_rain_risk.txt");

    assert_eq!(solve(input.lines()), 879);
}
