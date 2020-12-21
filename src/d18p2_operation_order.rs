use crate::util;
use regex::Captures;
use regex::Regex;

fn solve(input: &str) -> i64 {
    let regex1 = Regex::new(r"()(\d+) (\+) (\d+)()").unwrap();
    let regex2 = Regex::new(r"(^|\()(\d+) (\*) (\d+)($|\)| \*)").unwrap();
    let regex3 = Regex::new(r"\((\d+)\)").unwrap();

    let mut s = input.to_owned();
    while s.find(&['+', '*', '('][..]).is_some() {
        while let Some(r) = calcs(&regex1, &s) {
            s = r;
        }

        s = calcs(&regex2, &s).unwrap_or(s);
        s = regex3.replace(&s, "$1").as_ref().to_owned();
    }

    s.parse().unwrap()
}

fn calcs(regex: &Regex, s: &str) -> Option<String> {
    let r = regex
        .replace(s, |cap: &Captures| {
            cap[1].to_owned()
                + &calc(cap[2].parse().unwrap(), &cap[3], cap[4].parse().unwrap()).to_string()
                + &cap[5]
        })
        .as_ref()
        .to_owned();
    if r == s {
        None
    } else {
        Some(r.to_owned())
    }
}

fn calc(in1: i64, op: &str, in2: i64) -> i64 {
    match op {
        "+" => in1 + in2,
        "*" => in1 * in2,
        _ => panic!(op.to_owned()),
    }
}

#[test]
fn test_example() {
    assert_eq!(solve("2 * 3 + (4 * 5)"), 46);
    assert_eq!(solve("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(solve("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        solve("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
}

#[test]
fn test() {
    let input = util::read_input("d18_operation_order.txt");

    assert_eq!(input.lines().map(|l| solve(l)).sum::<i64>(), 60807587180737);
}
