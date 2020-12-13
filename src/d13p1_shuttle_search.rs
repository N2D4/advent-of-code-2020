use crate::util;
use itertools::Itertools;

fn solve(t: i32, buses: impl IntoIterator<Item = i32>) -> i32 {
    let res = buses
        .into_iter()
        .map(|b| (b, b - t % b))
        .min_by_key(|t| t.1)
        .unwrap();
    res.0 * res.1
}

#[test]
fn test_example() {
    assert_eq!(solve(939, vec![7, 13, 59, 31, 19]), 295);
}

#[test]
fn test() {
    let input = util::read_input("d13_shuttle_search.txt");
    let (t, buses) = input.lines().next_tuple().unwrap();

    assert_eq!(
        solve(
            t.parse().unwrap(),
            buses.split(",").map(|x| x.parse()).flatten()
        ),
        3966
    );
}
