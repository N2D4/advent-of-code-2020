use crate::util;
use iter::once;
use itertools::Itertools;
use std::collections::HashSet;
use std::iter;

fn solve(lines: Vec<&str>) -> i64 {
    lines
        .iter()
        .enumerate()
        .map(|(i, &line)| {
            run_program(
                lines[..i]
                    .iter()
                    .chain(once(
                        &match line.split(' ').next_tuple().unwrap() {
                            ("nop", a) => "jmp ".to_owned() + a,
                            ("jmp", a) => "nop ".to_owned() + a,
                            (op, a) => op.to_owned() + " " + a,
                        }
                        .as_ref(),
                    ))
                    .chain(lines[i + 1..].iter())
                    .copied()
                    .collect(),
            )
        })
        .filter_map(|x| x)
        .next()
        .unwrap()
}

fn run_program(lines: Vec<&str>) -> Option<i64> {
    let mut acc: i64 = 0;
    let mut pc: isize = 0;
    let mut executed: HashSet<isize> = HashSet::new();
    loop {
        if pc == lines.len() as isize {
            break Some(acc);
        } else if !executed.insert(pc) || pc < 0 || pc > lines.len() as isize {
            break None;
        } else {
            match lines[pc as usize].split(' ').next_tuple().unwrap() {
                ("nop", _) => (),
                ("acc", a) => acc += a.parse::<i64>().unwrap(),
                ("jmp", a) => pc += a.parse::<isize>().unwrap() - 1,
                (_, _) => panic!("Unknown operation!"),
            }
            pc += 1
        }
    }
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]),
        8
    );
}

#[test]
fn test() {
    let input = util::read_input("d8_handheld_halting.txt");

    assert_eq!(solve(input.lines().collect()), 1543);
}
