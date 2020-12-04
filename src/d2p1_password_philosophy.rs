use crate::util;
use regex::Regex;

fn solve(inputs: impl IntoIterator<Item = impl AsRef<str>>) -> usize {
    lazy_static! {
        static ref RGX: Regex =
            Regex::new(r"^(\d+)\-(\d+) ([a-z]): ([a-z]*)$").expect("regex doesn't compile!");
    }
    inputs
        .into_iter()
        .map(|s| {
            let str = s.as_ref().to_owned();
            let captures = RGX.captures_iter(&str).next().expect("no match!");
            (
                captures[1].parse::<usize>().unwrap(),
                captures[2].parse::<usize>().unwrap(),
                captures[3].to_owned(),
                captures[4].to_owned(),
            )
        })
        .filter(|t| {
            let c = t.3.match_indices(&t.2).count();
            t.0 <= c && c <= t.1
        })
        .count()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(&["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]),
        2
    );
}

#[test]
fn test() {
    let vec: Vec<String> = util::read_input("d2_password_philosophy.txt")
        .lines()
        .map(|x| String::from(x))
        .collect();

    assert_eq!(solve(&vec), 560);
}
