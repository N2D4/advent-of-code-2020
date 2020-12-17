use crate::util;
use itertools::Itertools;

fn solve<T: AsRef<[bool]>>(input: impl AsRef<[T]>, steps: usize) -> i32 {
    (0..steps)
        .fold(
            vec![input.as_ref().iter().map(|x| x.as_ref().to_vec()).collect()],
            |i, _| step(i),
        )
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.iter().map(|&z| if z { 1 } else { 0 }).sum::<i32>())
                .sum::<i32>()
        })
        .sum()
}

fn step(input: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    /*let stringified = input
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| {
                    y.iter()
                        .map(|&c| if c { '#' } else { '.' })
                        .collect::<String>()
                })
                .collect_vec()
        })
        .collect_vec();
    dbg!(stringified);*/

    let mut bordered = input.clone();
    bordered.insert(0, Vec::new());
    bordered.insert(1, Vec::new());
    bordered.push(Vec::new());
    bordered.push(Vec::new());
    for bplane in bordered.iter_mut() {
        bplane.insert(0, Vec::new());
        bplane.insert(1, Vec::new());
        while bplane.len() < input[0].len() + 4 {
            bplane.push(Vec::new());
        }
        for bline in bplane.iter_mut() {
            bline.insert(0, false);
            bline.insert(1, false);
            while bline.len() < input[0][0].len() + 4 {
                bline.push(false);
            }
        }
    }

    let mut out = Vec::new();
    for i in 0..input.len() + 2 {
        let mut oplane = Vec::new();
        for j in 0..input[0].len() + 2 {
            let mut oline = Vec::new();
            for k in 0..input[0][0].len() + 2 {
                let mut neighbours = 0;
                for id in 0..=2 {
                    for jd in 0..=2 {
                        for kd in 0..=2 {
                            if id * jd * kd != 1 && bordered[i + id][j + jd][k + kd] {
                                neighbours += 1;
                            }
                        }
                    }
                }
                oline.push(neighbours == 3 || (bordered[i + 1][j + 1][k + 1] && neighbours == 2));
            }
            oplane.push(oline);
        }
        out.push(oplane);
    }
    out
}

#[test]
fn test_examples() {
    assert_eq!(
        solve(
            vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
            6
        ),
        112
    );
}

#[test]
fn test() {
    assert_eq!(
        solve(
            util::read_input("d17_conway_cubes.txt")
                .lines()
                .map(|l| l.chars().map(|c| c == '#').collect_vec())
                .collect_vec(),
            6
        ),
        382
    );
}
