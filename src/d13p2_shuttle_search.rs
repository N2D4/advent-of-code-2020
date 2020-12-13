use crate::util;
use itertools::Itertools;

fn solve(buses: impl IntoIterator<Item = (i128, i128)>) -> i128 {
    buses
        .into_iter()
        .fold1(|(a, b), (c, d)| {
            println!("{:?} {:?}", (a, b), (c, d));
            (
                a * c,
                (0..c)
                    .map(|i| a * i + b.rem_euclid(a))
                    .find(|x| x.rem_euclid(c) == d.rem_euclid(c))
                    .unwrap(),
            )
        })
        .unwrap()
        .1
}

#[test]
fn test_example() {
    assert_eq!(
        solve(vec![(7, 0), (13, 12), (59, 55), (31, 25), (19, 12)]),
        1068781
    );
}

#[test]
fn test() {
    let input = util::read_input("d13_shuttle_search.txt");
    let buses = input.lines().nth(1).unwrap();

    assert_eq!(
        solve(
            buses
                .split(",")
                .map(|x| x.parse())
                .enumerate()
                .filter_map(|(i, res)| res.map(|v| (v, v - i as i128)).ok())
        ),
        800177252346225
    );
}
