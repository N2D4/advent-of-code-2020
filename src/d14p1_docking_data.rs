use crate::util;
use itertools::Itertools;
use std::collections::hash_map::HashMap;

fn solve(input: impl IntoIterator<Item = impl AsRef<str>>) -> i64 {
    let mut mmask: i64 = 0;
    let mut xmask: i64 = 0;
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for str_ar in input {
        let str = str_ar.as_ref();
        if str.starts_with("mask") {
            mmask = i64::from_str_radix(&str[7..].replace("X", "0"), 2).unwrap();
            xmask = i64::from_str_radix(&str[7..].replace("1", "0").replace("X", "1"), 2).unwrap();
        } else {
            let (lhs, rhs) = str.split("] = ").next_tuple().unwrap();
            mem.insert(
                lhs[4..].parse().unwrap(),
                rhs.parse::<i64>().unwrap() & xmask | mmask,
            );
        }
    }
    mem.iter().map(|x| x.1).sum()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]),
        165
    );
}

#[test]
fn test() {
    assert_eq!(
        solve(util::read_input("d14_docking_data.txt").lines()),
        11327140210986
    );
}
