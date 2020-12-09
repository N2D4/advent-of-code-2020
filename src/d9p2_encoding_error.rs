use crate::util;

fn solve(numbers: &[i64], c: usize) -> i64 {
    let invalid = find_invalid(numbers, c);
    for start in 0..numbers.len() {
        let mut sum = 0;
        for end in start..numbers.len() {
            sum += numbers[end];
            if sum == invalid {
                return numbers[start..end + 1].iter().min().unwrap()
                    + numbers[start..end + 1].iter().max().unwrap();
            }
        }
    }
    panic!()
}

fn find_invalid(numbers: &[i64], c: usize) -> i64 {
    'outer: for (i, &n) in numbers.iter().enumerate() {
        if i <= c {
            continue;
        }
        for a in &numbers[i - c..i] {
            for b in &numbers[i - c..i] {
                if a + b == n {
                    continue 'outer;
                }
            }
        }
        return n;
    }
    panic!();
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            &[
                35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                309, 576,
            ],
            5
        ),
        62
    );
}

#[test]
fn test() {
    let input = util::read_input("d9_encoding_error.txt");

    assert_eq!(
        solve(
            &input
                .lines()
                .map(|l| l.parse().unwrap())
                .collect::<Vec<i64>>(),
            25
        ),
        96081673
    );
}
