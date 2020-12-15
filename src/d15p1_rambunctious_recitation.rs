use crate::util;
use std::collections::HashMap;

fn solve(input: &mut Vec<usize>) -> usize {
    let mut set: HashMap<usize, usize> = input
        .iter()
        .enumerate()
        .map(|(a, b)| (*b, a + 1))
        .take(input.len() - 1)
        .collect();
    for i in input.len()..2020 {
        let last = *input.last().unwrap();
        input.push(i - *set.get(&last).unwrap_or(&i));
        set.insert(last, i);
    }
    *input.last().unwrap()
}

#[test]
fn test_examples() {
    assert_eq!(solve(&mut vec![0, 3, 6]), 436);
    assert_eq!(solve(&mut vec![1, 3, 2]), 1);
    assert_eq!(solve(&mut vec![2, 1, 3]), 10);
    assert_eq!(solve(&mut vec![1, 2, 3]), 27);
}

#[test]
fn test() {
    assert_eq!(
        solve(
            &mut util::read_input("d15_rambunctious_recitation.txt")
                .lines()
                .next()
                .unwrap()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect()
        ),
        758
    );
}
