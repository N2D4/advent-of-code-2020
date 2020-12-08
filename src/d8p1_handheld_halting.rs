use crate::util;
use itertools::Itertools;
use std::collections::HashSet;

fn solve(lines: Vec<&str>) -> i64 {
    let mut acc: i64 = 0;
    let mut pc: usize = 0;
    let mut executed: HashSet<usize> = HashSet::new();
    loop {
        if !executed.insert(pc) {
            break acc;
        }
        match lines[pc].split(' ').next_tuple().unwrap() {
            ("nop", _) => (),
            ("acc", a) => acc += a.parse::<i64>().unwrap(),
            ("jmp", a) => pc = (pc as isize + a.parse::<isize>().unwrap() - 1) as usize,
            (_, _) => panic!("Unknown operation!"),
        }
        pc += 1
    }
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]),
        5
    );
}

#[test]
fn test() {
    let input = util::read_input("d8_handheld_halting.txt");

    assert_eq!(solve(input.lines().collect()), 1600);
}
