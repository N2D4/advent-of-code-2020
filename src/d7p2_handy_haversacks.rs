use crate::util;
use itertools::Itertools;
use std::collections::HashMap;

fn solve(inputs: impl AsRef<str>) -> i64 {
    let map: HashMap<String, HashMap<String, i64>> = inputs
        .as_ref()
        .lines()
        .map(|l| {
            let (a, b) = l
                .replace("bags", "bag")
                .split(" contain ")
                .map(|s| s.to_owned())
                .next_tuple()
                .unwrap();
            (
                a.replace(' ', ""),
                b.split(", ")
                    .filter(|&s| s != "no other bag.")
                    .map(|s| {
                        (
                            s.replace(
                                &[' ', '.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'][..],
                                "",
                            )
                            .to_owned(),
                            s.split(' ').next().unwrap().parse().unwrap(),
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    let mut mem: HashMap<String, i64> = HashMap::new();

    populate_mem("shinygoldbag", &map, &mut mem) - 1
}

fn populate_mem<'a, 'b, 'c: 'a + 'b>(
    bag: impl AsRef<str>,
    inputs: &'a HashMap<String, HashMap<impl AsRef<str>, i64>>,
    mem: &'a mut HashMap<String, i64>,
) -> i64 {
    if !mem.contains_key(bag.as_ref()) {
        let r = inputs[bag.as_ref()]
            .iter()
            .map(|s| s.1 * populate_mem(s.0, inputs, mem))
            .sum::<i64>()
            + 1;
        mem.insert(bag.as_ref().to_owned(), r);
    }
    mem[bag.as_ref()]
}

#[test]
fn test_example() {
    assert_eq!(
        solve(
            r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"
        ),
        32
    );
}
#[test]
fn test_example_deep() {
    assert_eq!(
        solve(
            r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"
        ),
        126
    );
}

#[test]
fn test() {
    let input = util::read_input("d7_handy_haversacks.txt");

    assert_eq!(solve(input), 24867);
}
