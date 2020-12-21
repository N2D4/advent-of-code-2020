use crate::util;

fn solve(s: &str) -> i64 {
    return solve_replaced(s).0;
}

fn calc(in1: i64, op: &str, in2: i64) -> i64 {
    match op {
        "+" => in1 + in2,
        "*" => in1 * in2,
        _ => panic!(op.to_owned()),
    }
}

fn solve_replaced(input: impl AsRef<str>) -> (i64, String) {
    let mut s = " + ".to_owned() + input.as_ref();
    let mut res = 0;
    while !s.is_empty() {
        if &s[0..1] == ")" {
            s = s[1..].to_owned();
            break;
        }

        let tuple = if &s[3..4] == "(" {
            solve_replaced(&s[4..])
        } else {
            let until = s[3..].find(&[' ', ')'][..]).map_or(s.len(), |x| x + 3);
            (s[3..until].parse().unwrap(), s[until..].to_owned())
        };

        res = calc(res, &s[1..2], tuple.0);
        s = tuple.1;
    }
    (res, s.to_owned())
}

#[test]
fn test_example() {
    assert_eq!(solve("2 * 3 + (4 * 5)"), 26);
    assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
}

#[test]
fn test() {
    let input = util::read_input("d18_operation_order.txt");

    assert_eq!(input.lines().map(|l| solve(l)).sum::<i64>(), 855438643439);
}
