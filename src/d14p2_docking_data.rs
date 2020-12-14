use crate::util;
use itertools::Itertools;
use std::collections::hash_map::HashMap;

fn solve(input: impl IntoIterator<Item = impl AsRef<str>>) -> i64 {
    let mut mmask = 0;
    let mut xmask = 0;
    let mut mem: HashMap<i64, i64> = HashMap::new();
    for str_ar in input {
        let str = str_ar.as_ref();
        if str.starts_with("mask") {
            mmask = i64::from_str_radix(&str[7..].replace("X", "1"), 2).unwrap();
            xmask = i64::from_str_radix(&str[7..].replace("1", "0").replace("X", "1"), 2).unwrap();
        } else {
            let (lhs, rhs) = str.split("] = ").next_tuple().unwrap();
            let mut msk = 0;
            loop {
                mem.insert(
                    (lhs[4..].parse::<i64>().unwrap() | mmask) ^ msk,
                    rhs.parse().unwrap(),
                );

                msk = -(xmask ^ msk) & xmask;
                if msk == 0 {
                    break;
                }
            }
        }
    }
    mem.iter().map(|x| x.1).sum()
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]),
        208
    );
}

#[test]
fn test() {
    assert_eq!(
        solve(util::read_input("d14_docking_data.txt").lines()),
        2308180581795
    );
}
